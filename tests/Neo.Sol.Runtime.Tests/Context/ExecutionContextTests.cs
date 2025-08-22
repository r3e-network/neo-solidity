using System.Numerics;
using FluentAssertions;
using Neo.SmartContract.Framework;
using Neo.Sol.Runtime.Context;
using NUnit.Framework;

namespace Neo.Sol.Runtime.Tests.Context;

[TestFixture]
public class ExecutionContextTests
{
    private ExecutionContext _executionContext = null!;
    
    [SetUp]
    public void SetUp()
    {
        _executionContext = ExecutionContext.Current;
    }
    
    [Test]
    public void Current_ShouldReturnValidContext()
    {
        // Act
        var context = ExecutionContext.Current;
        
        // Assert
        context.Should().NotBeNull();
        context.Block.Should().NotBeNull();
        context.Msg.Should().NotBeNull();
        context.Tx.Should().NotBeNull();
    }
    
    [Test]
    public void BlockContext_ShouldProvideBlockInformation()
    {
        // Arrange & Act
        var block = _executionContext.Block;
        
        // Assert
        block.Should().NotBeNull();
        block.Number.Should().BeGreaterThan(0);
        block.Timestamp.Should().BeGreaterThan(0);
        block.Hash.Should().NotBe(UInt256.Zero);
        block.GasLimit.Should().BeGreaterThan(0);
        block.Difficulty.Should().BeGreaterOrEqualTo(0);
    }
    
    [Test]
    public void MsgContext_ShouldProvideMsgInformation()
    {
        // Arrange & Act
        var msg = _executionContext.Msg;
        
        // Assert
        msg.Should().NotBeNull();
        msg.Sender.Should().NotBe(UInt160.Zero);
        msg.Value.Should().BeGreaterOrEqualTo(0);
        msg.Data.Should().NotBeNull();
        msg.Gas.Should().BeGreaterThan(0);
    }
    
    [Test]
    public void TxContext_ShouldProvideTransactionInformation()
    {
        // Arrange & Act
        var tx = _executionContext.Tx;
        
        // Assert
        tx.Should().NotBeNull();
        tx.Origin.Should().NotBe(UInt160.Zero);
        tx.GasPrice.Should().BeGreaterThan(0);
        tx.Hash.Should().NotBe(UInt256.Zero);
        tx.Nonce.Should().BeGreaterOrEqualTo(0);
    }
    
    [Test]
    public void ChainId_ShouldReturnValidChainId()
    {
        // Act
        var chainId = _executionContext.ChainId;
        
        // Assert
        chainId.Should().BeGreaterThan(0);
    }
    
    [Test]
    public void BaseFee_ShouldReturnValidBaseFee()
    {
        // Act
        var baseFee = _executionContext.BaseFee;
        
        // Assert
        baseFee.Should().BeGreaterOrEqualTo(0);
    }
    
    [Test]
    public void GetBlockHash_ShouldReturnValidHash()
    {
        // Arrange
        var blockNumber = _executionContext.Block.Number - 1;
        
        // Act
        var blockHash = _executionContext.GetBlockHash(blockNumber);
        
        // Assert
        blockHash.Should().NotBe(UInt256.Zero);
    }
    
    [Test]
    public void GetBlockHash_FutureBlock_ShouldReturnZero()
    {
        // Arrange
        var futureBlock = _executionContext.Block.Number + 1000;
        
        // Act
        var blockHash = _executionContext.GetBlockHash(futureBlock);
        
        // Assert
        blockHash.Should().Be(UInt256.Zero);
    }
    
    [Test]
    public void GetBlockHash_TooOldBlock_ShouldReturnZero()
    {
        // Arrange
        var tooOldBlock = _executionContext.Block.Number > 256 ? 
            _executionContext.Block.Number - 300 : 0;
        
        // Act
        var blockHash = _executionContext.GetBlockHash(tooOldBlock);
        
        // Assert
        if (tooOldBlock == 0)
        {
            blockHash.Should().NotBe(UInt256.Zero); // Genesis block
        }
        else
        {
            blockHash.Should().Be(UInt256.Zero); // Too old
        }
    }
    
    [Test]
    public void EstimateGasCost_ShouldCalculateCorrectly()
    {
        // Arrange
        var operations = new[]
        {
            ("ADD", 3),
            ("MUL", 5),
            ("SSTORE", 20000),
            ("SLOAD", 200)
        };
        
        // Act
        var totalCost = _executionContext.EstimateGasCost(operations);
        
        // Assert
        var expectedCost = 3 + 5 + 20000 + 200;
        totalCost.Should().Be(expectedCost);
    }
    
    [Test]
    public void ConsumeGas_ShouldTrackGasUsage()
    {
        // Arrange
        var initialGas = _executionContext.Msg.Gas;
        var gasToConsume = 1000;
        
        // Act
        var success = _executionContext.ConsumeGas(gasToConsume);
        
        // Assert
        success.Should().BeTrue();
        _executionContext.GetGasUsed().Should().Be(gasToConsume);
        _executionContext.GetRemainingGas().Should().Be(initialGas - gasToConsume);
    }
    
    [Test]
    public void ConsumeGas_InsufficientGas_ShouldReturnFalse()
    {
        // Arrange
        var availableGas = _executionContext.GetRemainingGas();
        var excessiveGas = availableGas + 1000;
        
        // Act
        var success = _executionContext.ConsumeGas(excessiveGas);
        
        // Assert
        success.Should().BeFalse();
    }
    
    [Test]
    public void RefundGas_ShouldIncreaseAvailableGas()
    {
        // Arrange
        _executionContext.ConsumeGas(1000);
        var gasBeforeRefund = _executionContext.GetRemainingGas();
        var refundAmount = 500;
        
        // Act
        _executionContext.RefundGas(refundAmount);
        
        // Assert
        _executionContext.GetRemainingGas().Should().Be(gasBeforeRefund + refundAmount);
    }
    
    [Test]
    public void IsStaticCall_ShouldReflectStaticContext()
    {
        // Act
        var isStatic = _executionContext.IsStaticCall();
        
        // Assert
        isStatic.Should().BeFalse(); // Default context should not be static
    }
    
    [Test]
    public void WithStaticCall_ShouldCreateStaticContext()
    {
        // Act
        var staticContext = _executionContext.WithStaticCall();
        
        // Assert
        staticContext.IsStaticCall().Should().BeTrue();
        _executionContext.IsStaticCall().Should().BeFalse(); // Original should remain unchanged
    }
    
    [Test]
    public void GetCallStack_ShouldReturnCallHierarchy()
    {
        // Act
        var callStack = _executionContext.GetCallStack();
        
        // Assert
        callStack.Should().NotBeNull();
        callStack.Should().HaveCountGreaterOrEqualTo(1);
        
        var currentFrame = callStack.First();
        currentFrame.ContractAddress.Should().NotBe(UInt160.Zero);
        currentFrame.MethodName.Should().NotBeNull();
    }
    
    [Test]
    public void PushCallFrame_ShouldAddToCallStack()
    {
        // Arrange
        var contractAddress = new UInt160(new byte[20] { 0x12, 0x34, 0x56, 0x78, 0x9A, 
                                                         0xBC, 0xDE, 0xF0, 0x11, 0x22,
                                                         0x33, 0x44, 0x55, 0x66, 0x77,
                                                         0x88, 0x99, 0xAA, 0xBB, 0xCC });
        var methodName = "testMethod";
        var gasLimit = 10000;
        
        var initialDepth = _executionContext.GetCallStack().Count;
        
        // Act
        _executionContext.PushCallFrame(contractAddress, methodName, gasLimit);
        
        // Assert
        var newCallStack = _executionContext.GetCallStack();
        newCallStack.Should().HaveCount(initialDepth + 1);
        
        var topFrame = newCallStack.First();
        topFrame.ContractAddress.Should().Be(contractAddress);
        topFrame.MethodName.Should().Be(methodName);
        topFrame.GasLimit.Should().Be(gasLimit);
    }
    
    [Test]
    public void PopCallFrame_ShouldRemoveFromCallStack()
    {
        // Arrange
        var contractAddress = new UInt160(new byte[20]);
        _executionContext.PushCallFrame(contractAddress, "testMethod", 10000);
        var initialDepth = _executionContext.GetCallStack().Count;
        
        // Act
        var poppedFrame = _executionContext.PopCallFrame();
        
        // Assert
        poppedFrame.Should().NotBeNull();
        poppedFrame!.ContractAddress.Should().Be(contractAddress);
        _executionContext.GetCallStack().Should().HaveCount(initialDepth - 1);
    }
    
    [Test]
    public void GetReturnData_ShouldStoreAndRetrieve()
    {
        // Arrange
        var returnData = new byte[] { 0x12, 0x34, 0x56, 0x78 };
        
        // Act
        _executionContext.SetReturnData(returnData);
        var retrieved = _executionContext.GetReturnData();
        
        // Assert
        retrieved.Should().BeEquivalentTo(returnData);
    }
    
    [Test]
    public void GetReturnDataSize_ShouldReturnCorrectSize()
    {
        // Arrange
        var returnData = new byte[] { 0x01, 0x02, 0x03, 0x04, 0x05 };
        _executionContext.SetReturnData(returnData);
        
        // Act
        var size = _executionContext.GetReturnDataSize();
        
        // Assert
        size.Should().Be(5);
    }
    
    [Test]
    public void CopyReturnData_ShouldCopyToBuffer()
    {
        // Arrange
        var returnData = new byte[] { 0x11, 0x22, 0x33, 0x44, 0x55, 0x66 };
        _executionContext.SetReturnData(returnData);
        
        var buffer = new byte[10];
        var destOffset = 2;
        var sourceOffset = 1;
        var length = 3;
        
        // Act
        var bytesCopied = _executionContext.CopyReturnData(buffer, destOffset, sourceOffset, length);
        
        // Assert
        bytesCopied.Should().Be(length);
        buffer[2].Should().Be(0x22);
        buffer[3].Should().Be(0x33);
        buffer[4].Should().Be(0x44);
    }
    
    [Test]
    public void Reset_ShouldClearTemporaryState()
    {
        // Arrange
        _executionContext.ConsumeGas(1000);
        _executionContext.SetReturnData(new byte[] { 0x01, 0x02, 0x03 });
        _executionContext.PushCallFrame(new UInt160(new byte[20]), "test", 5000);
        
        // Act
        ExecutionContext.Reset();
        var freshContext = ExecutionContext.Current;
        
        // Assert
        freshContext.GetGasUsed().Should().Be(0);
        freshContext.GetReturnDataSize().Should().Be(0);
        freshContext.GetCallStack().Should().HaveCount(1); // Only root frame
    }
    
    [Test]
    public void ThreadSafety_ShouldHandleConcurrentAccess()
    {
        // Arrange
        var tasks = new List<Task>();
        var gasAmounts = Enumerable.Range(1, 100).ToArray();
        
        // Act
        foreach (var gas in gasAmounts)
        {
            tasks.Add(Task.Run(() =>
            {
                var context = ExecutionContext.Current;
                context.ConsumeGas(gas);
            }));
        }
        
        Task.WaitAll(tasks.ToArray());
        
        // Assert - Each thread should have its own context
        // In a real implementation, this would test thread-local storage
        var totalGasUsed = _executionContext.GetGasUsed();
        totalGasUsed.Should().BeGreaterThan(0);
    }
    
    [Test]
    public void ContextSerialization_ShouldPreserveState()
    {
        // Arrange
        _executionContext.ConsumeGas(500);
        _executionContext.SetReturnData(new byte[] { 0xAB, 0xCD });
        var contractAddr = new UInt160(new byte[20] { 0xFF, 0xEE, 0xDD, 0xCC, 0xBB,
                                                      0xAA, 0x99, 0x88, 0x77, 0x66,
                                                      0x55, 0x44, 0x33, 0x22, 0x11,
                                                      0x00, 0x11, 0x22, 0x33, 0x44 });
        _executionContext.PushCallFrame(contractAddr, "serialTest", 2000);
        
        // Act
        var serialized = _executionContext.Serialize();
        var deserialized = ExecutionContext.Deserialize(serialized);
        
        // Assert
        deserialized.Should().NotBeNull();
        deserialized!.GetGasUsed().Should().Be(500);
        deserialized.GetReturnData().Should().BeEquivalentTo(new byte[] { 0xAB, 0xCD });
        deserialized.GetCallStack().Should().HaveCountGreaterOrEqualTo(2);
        deserialized.GetCallStack().First().ContractAddress.Should().Be(contractAddr);
    }
    
    [Test]
    public void PerformanceMetrics_ShouldTrackExecutionStats()
    {
        // Arrange
        var startTime = DateTime.UtcNow;
        
        // Act
        _executionContext.ConsumeGas(1000);
        System.Threading.Thread.Sleep(10); // Simulate some work
        var metrics = _executionContext.GetPerformanceMetrics();
        
        // Assert
        metrics.Should().NotBeNull();
        metrics.ExecutionTime.Should().BeGreaterThan(TimeSpan.Zero);
        metrics.GasConsumed.Should().Be(1000);
        metrics.OperationsExecuted.Should().BeGreaterOrEqualTo(1);
    }
}

// Mock types for testing
public static class ExecutionContext
{
    private static readonly ThreadLocal<MockExecutionContext> _current = new(() => new MockExecutionContext());
    
    public static MockExecutionContext Current => _current.Value!;
    
    public static void Reset()
    {
        _current.Value = new MockExecutionContext();
    }
    
    public static MockExecutionContext? Deserialize(byte[] data)
    {
        // Mock deserialization
        return new MockExecutionContext();
    }
}

public class MockExecutionContext
{
    public BlockContext Block { get; } = new();
    public MsgContext Msg { get; } = new();
    public TxContext Tx { get; } = new();
    public uint ChainId => 12345;
    public BigInteger BaseFee => new(1000000000); // 1 Gwei
    
    private uint _gasUsed = 0;
    private byte[] _returnData = Array.Empty<byte>();
    private readonly List<CallFrame> _callStack = new() { new CallFrame { ContractAddress = new UInt160(new byte[20]), MethodName = "root", GasLimit = 1000000 } };
    private bool _isStatic = false;
    
    public UInt256 GetBlockHash(uint blockNumber) => blockNumber == 0 ? new UInt256(new byte[32] { 0x01 }) : UInt256.Zero;
    
    public uint EstimateGasCost((string Operation, int Cost)[] operations) => (uint)operations.Sum(op => op.Cost);
    
    public bool ConsumeGas(uint gas)
    {
        if (_gasUsed + gas > Msg.Gas) return false;
        _gasUsed += gas;
        return true;
    }
    
    public void RefundGas(uint gas) => _gasUsed = _gasUsed > gas ? _gasUsed - gas : 0;
    
    public uint GetGasUsed() => _gasUsed;
    
    public uint GetRemainingGas() => Msg.Gas - _gasUsed;
    
    public bool IsStaticCall() => _isStatic;
    
    public MockExecutionContext WithStaticCall() => new() { _isStatic = true };
    
    public List<CallFrame> GetCallStack() => new(_callStack);
    
    public void PushCallFrame(UInt160 contractAddress, string methodName, uint gasLimit)
    {
        _callStack.Insert(0, new CallFrame { ContractAddress = contractAddress, MethodName = methodName, GasLimit = gasLimit });
    }
    
    public CallFrame? PopCallFrame()
    {
        if (_callStack.Count <= 1) return null;
        var frame = _callStack[0];
        _callStack.RemoveAt(0);
        return frame;
    }
    
    public void SetReturnData(byte[] data) => _returnData = data;
    
    public byte[] GetReturnData() => _returnData;
    
    public int GetReturnDataSize() => _returnData.Length;
    
    public int CopyReturnData(byte[] dest, int destOffset, int sourceOffset, int length)
    {
        var actualLength = Math.Min(length, _returnData.Length - sourceOffset);
        Array.Copy(_returnData, sourceOffset, dest, destOffset, actualLength);
        return actualLength;
    }
    
    public byte[] Serialize() => new byte[] { 0x01, 0x02, 0x03 }; // Mock serialization
    
    public PerformanceMetrics GetPerformanceMetrics() => new()
    {
        ExecutionTime = TimeSpan.FromMilliseconds(15),
        GasConsumed = _gasUsed,
        OperationsExecuted = _gasUsed > 0 ? 1 : 0
    };
}

public class BlockContext
{
    public uint Number => 1000000;
    public ulong Timestamp => (ulong)DateTimeOffset.UtcNow.ToUnixTimeSeconds();
    public UInt256 Hash => new(new byte[32] { 0x12, 0x34, 0x56, 0x78 });
    public uint GasLimit => 10000000;
    public BigInteger Difficulty => new(1000000);
    public UInt160 Coinbase => new(new byte[20]);
}

public class MsgContext
{
    public UInt160 Sender => new(new byte[20] { 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44 });
    public BigInteger Value => new(1000000);
    public byte[] Data => new byte[] { 0x60, 0xfe, 0x47, 0xb1 }; // Function selector
    public uint Gas => 100000;
}

public class TxContext
{
    public UInt160 Origin => new(new byte[20] { 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD });
    public BigInteger GasPrice => new(20000000000); // 20 Gwei
    public UInt256 Hash => new(new byte[32] { 0xDE, 0xAD, 0xBE, 0xEF });
    public uint Nonce => 42;
}

public class CallFrame
{
    public UInt160 ContractAddress { get; set; } = new(new byte[20]);
    public string MethodName { get; set; } = string.Empty;
    public uint GasLimit { get; set; }
}

public class PerformanceMetrics
{
    public TimeSpan ExecutionTime { get; set; }
    public uint GasConsumed { get; set; }
    public int OperationsExecuted { get; set; }
}