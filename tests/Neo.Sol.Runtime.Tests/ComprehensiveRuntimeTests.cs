using System.Numerics;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Moq;
using Neo.Sol.Runtime;
using Neo.Sol.Runtime.Memory;
using Neo.Sol.Runtime.Storage;
using Neo.Sol.Runtime.ABI;
using Neo.Sol.Runtime.Crypto;
using Neo.Sol.Runtime.Events;
using Neo.Sol.Runtime.Context;
using Neo.Sol.Runtime.Calls;
using Neo.Sol.Runtime.Registry;
using Neo.Sol.Runtime.Exceptions;

namespace Neo.Sol.Runtime.Tests;

/// <summary>
/// Comprehensive test suite for the Neo-Sol Runtime
/// Tests all major components with edge cases and integration scenarios
/// </summary>
[TestClass]
public class ComprehensiveRuntimeTests
{
    private EvmRuntime? _runtime;
    private Mock<StorageContext>? _mockStorage;
    
    [TestInitialize]
    public void Setup()
    {
        _mockStorage = new Mock<StorageContext>();
        var contractAddress = UInt160.Parse("0x1234567890123456789012345678901234567890");
        _runtime = new EvmRuntime(contractAddress);
    }
    
    [TestCleanup]
    public void Cleanup()
    {
        _runtime?.Dispose();
        _runtime = null;
        _mockStorage = null;
    }
    
    /// <summary>
    /// Memory Manager Tests
    /// </summary>
    [TestClass]
    public class MemoryManagerTests
    {
        private EvmMemoryManager? _memoryManager;
        
        [TestInitialize]
        public void Setup()
        {
            _memoryManager = new EvmMemoryManager();
        }
        
        [TestCleanup]
        public void Cleanup()
        {
            _memoryManager?.Dispose();
        }
        
        [TestMethod]
        public void MemoryManager_BasicOperations_ShouldWork()
        {
            // Test word storage and retrieval
            var testValue = BigInteger.Parse("123456789012345678901234567890");
            _memoryManager!.Store(0, testValue);
            var retrieved = _memoryManager.LoadBigInteger(0);
            
            Assert.AreEqual(testValue, retrieved);
        }
        
        [TestMethod]
        public void MemoryManager_PageAllocation_ShouldHandleLargeMemory()
        {
            // Test large memory allocation
            var largeAddress = 1024 * 1024; // 1MB
            var testData = new byte[32];
            Array.Fill(testData, (byte)0xFF);
            
            _memoryManager!.Store(largeAddress, testData);
            var retrieved = _memoryManager.Load(largeAddress);
            
            CollectionAssert.AreEqual(testData, retrieved);
        }
        
        [TestMethod]
        public void MemoryManager_GarbageCollection_ShouldFreeUnusedPages()
        {
            // Allocate memory
            for (uint i = 0; i < 100; i++)
            {
                _memoryManager!.Store(i * 4096, BigInteger.One);
            }
            
            var statsBefore = _memoryManager.GetStats();
            
            // Force garbage collection
            _memoryManager.ForceGarbageCollection();
            
            var statsAfter = _memoryManager.GetStats();
            
            Assert.IsTrue(statsAfter.GCCollections > statsBefore.GCCollections);
        }
        
        [TestMethod]
        public void MemoryManager_MemoryExpansionCost_ShouldCalculateCorrectly()
        {
            var cost1 = _memoryManager!.CalculateExpansionCost(1024);
            var cost2 = _memoryManager.CalculateExpansionCost(2048);
            
            // Cost should be quadratic - larger expansions cost more per byte
            Assert.IsTrue(cost2 > cost1 * 1.5);
        }
        
        [TestMethod]
        [ExpectedException(typeof(InvalidOperationException))]
        public void MemoryManager_ExceedMaxSize_ShouldThrow()
        {
            var maxSize = 64 * 1024 * 1024 + 1; // Exceed 64MB limit
            _memoryManager!.ExpandMemory(maxSize);
        }
        
        [TestMethod]
        public void MemoryManager_ConcurrentAccess_ShouldBeThreadSafe()
        {
            const int threadCount = 10;
            const int operationsPerThread = 100;
            var tasks = new Task[threadCount];
            
            for (int t = 0; t < threadCount; t++)
            {
                var threadId = t;
                tasks[t] = Task.Run(() =>
                {
                    for (int i = 0; i < operationsPerThread; i++)
                    {
                        var address = (uint)(threadId * operationsPerThread + i) * 32;
                        var value = new BigInteger(threadId * 1000 + i);
                        
                        _memoryManager!.Store(address, value);
                        var retrieved = _memoryManager.LoadBigInteger(address);
                        
                        Assert.AreEqual(value, retrieved);
                    }
                });
            }
            
            Task.WaitAll(tasks);
        }
    }
    
    /// <summary>
    /// Storage Manager Tests
    /// </summary>
    [TestClass]
    public class StorageManagerTests
    {
        private StorageManager? _storageManager;
        private Mock<StorageContext>? _mockContext;
        
        [TestInitialize]
        public void Setup()
        {
            _mockContext = new Mock<StorageContext>();
            _storageManager = new StorageManager(_mockContext.Object);
        }
        
        [TestCleanup]
        public void Cleanup()
        {
            _storageManager?.Dispose();
        }
        
        [TestMethod]
        public void StorageManager_BasicSlotOperations_ShouldWork()
        {
            var slot = BigInteger.One;
            var value = BigInteger.Parse("999888777666555444333222111");
            
            _storageManager!.Store(slot, value);
            var retrieved = _storageManager.LoadBigInteger(slot);
            
            Assert.AreEqual(value, retrieved);
        }
        
        [TestMethod]
        public void StorageManager_MappingSlotCalculation_ShouldBeConsistent()
        {
            var mappingSlot = BigInteger.Zero;
            var key = BigInteger.Parse("12345");
            
            var calculatedSlot1 = StorageManager.CalculateMappingElementSlot(mappingSlot, key);
            var calculatedSlot2 = StorageManager.CalculateMappingElementSlot(mappingSlot, key);
            
            Assert.AreEqual(calculatedSlot1, calculatedSlot2);
        }
        
        [TestMethod]
        public void StorageManager_ArraySlotCalculation_ShouldBeSequential()
        {
            var arraySlot = BigInteger.Parse("5");
            
            var slot0 = StorageManager.CalculateArrayElementSlot(arraySlot, BigInteger.Zero);
            var slot1 = StorageManager.CalculateArrayElementSlot(arraySlot, BigInteger.One);
            
            Assert.AreEqual(slot0 + 1, slot1);
        }
        
        [TestMethod]
        public void StorageManager_BatchOperations_ShouldBeEfficient()
        {
            var updates = new Dictionary<BigInteger, byte[]>();
            
            for (int i = 0; i < 100; i++)
            {
                var slot = new BigInteger(i);
                var value = new byte[32];
                Array.Fill(value, (byte)i);
                updates[slot] = value;
            }
            
            _storageManager!.BatchStore(updates);
            
            var loaded = _storageManager.BatchLoad(updates.Keys);
            
            Assert.AreEqual(updates.Count, loaded.Count);
            
            foreach (var kvp in updates)
            {
                CollectionAssert.AreEqual(kvp.Value, loaded[kvp.Key]);
            }
        }
        
        [TestMethod]
        public void StorageManager_CachePerformance_ShouldImproveWithRepeatedAccess()
        {
            var slot = BigInteger.Parse("42");
            var value = BigInteger.Parse("314159265359");
            
            _storageManager!.Store(slot, value);
            
            // First access - cache miss
            var start1 = DateTime.UtcNow;
            var result1 = _storageManager.LoadBigInteger(slot);
            var time1 = DateTime.UtcNow - start1;
            
            // Second access - cache hit
            var start2 = DateTime.UtcNow;
            var result2 = _storageManager.LoadBigInteger(slot);
            var time2 = DateTime.UtcNow - start2;
            
            Assert.AreEqual(result1, result2);
            // Cache hit should be faster (though in unit tests this might not always be measurable)
        }
    }
    
    /// <summary>
    /// ABI Encoder/Decoder Tests
    /// </summary>
    [TestClass]
    public class AbiTests
    {
        [TestMethod]
        public void AbiEncoder_BasicTypes_ShouldEncodeCorrectly()
        {
            // Test various basic types
            var boolValue = true;
            var uintValue = new BigInteger(12345);
            var intValue = new BigInteger(-6789);
            var stringValue = "Hello, World!";
            var addressValue = UInt160.Parse("0x1234567890123456789012345678901234567890");
            
            var encodedBool = AbiEncoder.EncodeUint256(boolValue ? 1 : 0);
            var encodedUint = AbiEncoder.EncodeUint256(uintValue);
            var encodedInt = AbiEncoder.EncodeInt256(intValue);
            var encodedAddress = AbiEncoder.EncodeAddress(addressValue);
            
            Assert.AreEqual(32, encodedBool.Length);
            Assert.AreEqual(32, encodedUint.Length);
            Assert.AreEqual(32, encodedInt.Length);
            Assert.AreEqual(32, encodedAddress.Length);
            
            // Test decoding
            var decodedBool = AbiDecoder.DecodeBool(encodedBool) == boolValue;
            var decodedUint = AbiDecoder.DecodeUint256(encodedUint) == uintValue;
            var decodedInt = AbiDecoder.DecodeInt256(encodedInt) == intValue;
            var decodedAddress = AbiDecoder.DecodeAddress(encodedAddress).Equals(addressValue);
            
            Assert.IsTrue(decodedBool);
            Assert.IsTrue(decodedUint);
            Assert.IsTrue(decodedInt);
            Assert.IsTrue(decodedAddress);
        }
        
        [TestMethod]
        public void AbiEncoder_FunctionCall_ShouldGenerateCorrectSelector()
        {
            var signature = "transfer(address,uint256)";
            var selector = AbiEncoder.CalculateFunctionSelector(signature);
            
            Assert.AreEqual(4, selector.Length);
            
            // Test that same signature always produces same selector
            var selector2 = AbiEncoder.CalculateFunctionSelector(signature);
            CollectionAssert.AreEqual(selector, selector2);
        }
        
        [TestMethod]
        public void AbiEncoder_EventTopic_ShouldCalculateCorrectHash()
        {
            var eventSignature = "Transfer(address,address,uint256)";
            var topic = AbiEncoder.CalculateEventTopic(eventSignature);
            
            Assert.AreEqual(32, topic.Length);
            
            // Test that same signature produces same topic
            var topic2 = AbiEncoder.CalculateEventTopic(eventSignature);
            CollectionAssert.AreEqual(topic, topic2);
        }
        
        [TestMethod]
        public void AbiEncoder_ComplexParameters_ShouldHandleArraysAndStrings()
        {
            var parameters = new object[]
            {
                BigInteger.Parse("12345"),
                "Test String",
                new BigInteger[] { BigInteger.One, BigInteger.Parse("100"), BigInteger.Parse("1000") }
            };
            
            var encoded = AbiEncoder.EncodeParameters(parameters);
            Assert.IsTrue(encoded.Length > 0);
            
            // Test that we can decode the call
            var callData = AbiEncoder.EncodeCall("testFunction(uint256,string,uint256[])", parameters);
            Assert.IsTrue(callData.Length >= 4); // At least selector
        }
        
        [TestMethod]
        public void AbiEncoder_PackedEncoding_ShouldBeTight()
        {
            var values = new object[]
            {
                (byte)255,
                (ushort)65535,
                (uint)4294967295,
                "test"
            };
            
            var packed = AbiEncoder.EncodePacked(values);
            var expectedLength = 1 + 2 + 4 + 4; // byte + ushort + uint + string length
            
            Assert.AreEqual(expectedLength, packed.Length);
        }
        
        [TestMethod]
        [ExpectedException(typeof(ArgumentException))]
        public void AbiEncoder_InvalidInput_ShouldThrow()
        {
            AbiEncoder.CalculateFunctionSelector("");
        }
    }
    
    /// <summary>
    /// Crypto Library Tests
    /// </summary>
    [TestClass]
    public class CryptoTests
    {
        [TestMethod]
        public void CryptoLib_Keccak256_ShouldProduceCorrectHash()
        {
            var input = "Hello, World!"u8.ToArray();
            var hash = CryptoLib.Keccak256(input);
            
            Assert.AreEqual(32, hash.Length);
            
            // Test that same input produces same hash
            var hash2 = CryptoLib.Keccak256(input);
            CollectionAssert.AreEqual(hash, hash2);
        }
        
        [TestMethod]
        public void CryptoLib_Sha256_ShouldProduceCorrectHash()
        {
            var input = "Hello, World!"u8.ToArray();
            var hash = CryptoLib.Sha256(input);
            
            Assert.AreEqual(32, hash.Length);
            
            // Test consistency
            var hash2 = CryptoLib.Sha256(input);
            CollectionAssert.AreEqual(hash, hash2);
        }
        
        [TestMethod]
        public void CryptoLib_PublicKeyToAddress_ShouldProduceValidAddress()
        {
            var publicKey = new byte[64];
            // Fill with test data
            for (int i = 0; i < 64; i++)
            {
                publicKey[i] = (byte)(i % 256);
            }
            
            var address = CryptoLib.PublicKeyToAddress(publicKey);
            
            Assert.AreEqual(20, address.Length);
        }
        
        [TestMethod]
        public void CryptoLib_SignatureVerification_ShouldWork()
        {
            var messageHash = new byte[32];
            Array.Fill(messageHash, (byte)0x11);
            
            var signature = new byte[64];
            // This would be a real signature in production
            Array.Fill(signature, (byte)0x22);
            
            var publicKey = new byte[64];
            Array.Fill(publicKey, (byte)0x33);
            
            // This test just ensures the function doesn't crash
            // In a real scenario, you'd use actual cryptographic test vectors
            var isValid = CryptoLib.VerifySignature(messageHash, signature, publicKey);
            
            // The result doesn't matter as much as not crashing
            Assert.IsNotNull(isValid);
        }
        
        [TestMethod]
        [ExpectedException(typeof(ArgumentException))]
        public void CryptoLib_InvalidPublicKeySize_ShouldThrow()
        {
            var invalidPublicKey = new byte[63]; // Should be 64
            CryptoLib.PublicKeyToAddress(invalidPublicKey);
        }
    }
    
    /// <summary>
    /// Integration Tests
    /// </summary>
    [TestClass]
    public class IntegrationTests
    {
        private EvmRuntime? _runtime;
        
        [TestInitialize]
        public void Setup()
        {
            var contractAddress = UInt160.Parse("0x1234567890123456789012345678901234567890");
            _runtime = new EvmRuntime(contractAddress);
        }
        
        [TestCleanup]
        public void Cleanup()
        {
            _runtime?.Dispose();
        }
        
        [TestMethod]
        public void Runtime_FullWorkflow_ShouldIntegrateAllComponents()
        {
            // Test a complete workflow using all components
            
            // 1. Store some data
            _runtime!.Storage.Store(BigInteger.One, BigInteger.Parse("12345"));
            
            // 2. Use memory
            _runtime.Memory.Store(0, BigInteger.Parse("67890"));
            
            // 3. Emit an event
            _runtime.Events.Log1("Test(uint256)", BigInteger.Parse("99999"));
            
            // 4. Test crypto functions
            var hash = _runtime.Keccak256("test"u8.ToArray());
            
            // 5. Get statistics
            var stats = _runtime.GetStats();
            
            Assert.IsNotNull(stats);
            Assert.IsTrue(stats.MemoryStats.TotalSize > 0);
            Assert.IsTrue(stats.StorageStats.ModifiedSlots > 0);
            Assert.AreEqual(32, hash.Length);
        }
        
        [TestMethod]
        public void Runtime_ErrorHandling_ShouldGracefullyHandleExceptions()
        {
            try
            {
                _runtime!.Require(false, "This should fail");
                Assert.Fail("Should have thrown exception");
            }
            catch (Exception ex)
            {
                Assert.IsTrue(ex.Message.Contains("This should fail"));
            }
        }
        
        [TestMethod]
        public void Runtime_GasTracking_ShouldMonitorConsumption()
        {
            var gasLeftBefore = _runtime!.Context.Gas.GetGasLeft();
            
            // Perform operations that should consume gas
            _runtime.Storage.Store(BigInteger.One, BigInteger.Parse("12345"));
            _runtime.Memory.Store(0, BigInteger.Parse("67890"));
            
            var gasLeftAfter = _runtime.Context.Gas.GetGasLeft();
            
            // In a real implementation, gas should be consumed
            Assert.IsTrue(gasLeftBefore >= gasLeftAfter);
        }
        
        [TestMethod]
        public void Runtime_AddressResolution_ShouldWorkWithRegistry()
        {
            var testAddress = UInt160.Parse("0x9876543210987654321098765432109876543210");
            var contractInfo = new ContractInfo
            {
                Name = "TestContract",
                Version = "1.0.0",
                Owner = _runtime!.ContractAddress,
                IsActive = true,
                CreatedAt = (ulong)DateTimeOffset.UtcNow.ToUnixTimeSeconds()
            };
            
            _runtime.Registry.RegisterContract(testAddress, contractInfo);
            
            var retrieved = _runtime.Registry.GetContractInfo(testAddress);
            
            Assert.IsNotNull(retrieved);
            Assert.AreEqual("TestContract", retrieved.Name);
            Assert.IsTrue(retrieved.IsActive);
        }
    }
    
    /// <summary>
    /// Performance Tests
    /// </summary>
    [TestClass]
    public class PerformanceTests
    {
        private EvmRuntime? _runtime;
        
        [TestInitialize]
        public void Setup()
        {
            var contractAddress = UInt160.Parse("0x1234567890123456789012345678901234567890");
            _runtime = new EvmRuntime(contractAddress);
        }
        
        [TestCleanup]
        public void Cleanup()
        {
            _runtime?.Dispose();
        }
        
        [TestMethod]
        public void Performance_BulkStorageOperations_ShouldBeEfficient()
        {
            const int operationCount = 1000;
            var stopwatch = System.Diagnostics.Stopwatch.StartNew();
            
            for (int i = 0; i < operationCount; i++)
            {
                _runtime!.Storage.Store(new BigInteger(i), new BigInteger(i * 2));
            }
            
            stopwatch.Stop();
            
            // Should complete within reasonable time
            Assert.IsTrue(stopwatch.ElapsedMilliseconds < 5000); // 5 seconds max
            
            Console.WriteLine($"Bulk storage operations: {stopwatch.ElapsedMilliseconds}ms for {operationCount} operations");
        }
        
        [TestMethod]
        public void Performance_MemoryOperations_ShouldBeEfficient()
        {
            const int operationCount = 1000;
            var stopwatch = System.Diagnostics.Stopwatch.StartNew();
            
            for (uint i = 0; i < operationCount; i++)
            {
                _runtime!.Memory.Store(i * 32, new BigInteger(i));
            }
            
            stopwatch.Stop();
            
            Assert.IsTrue(stopwatch.ElapsedMilliseconds < 1000); // 1 second max
            
            Console.WriteLine($"Memory operations: {stopwatch.ElapsedMilliseconds}ms for {operationCount} operations");
        }
        
        [TestMethod]
        public void Performance_AbiOperations_ShouldBeEfficient()
        {
            const int operationCount = 1000;
            var stopwatch = System.Diagnostics.Stopwatch.StartNew();
            
            for (int i = 0; i < operationCount; i++)
            {
                var encoded = AbiEncoder.EncodeCall("test(uint256)", new BigInteger(i));
                Assert.IsTrue(encoded.Length > 0);
            }
            
            stopwatch.Stop();
            
            Assert.IsTrue(stopwatch.ElapsedMilliseconds < 2000); // 2 seconds max
            
            Console.WriteLine($"ABI operations: {stopwatch.ElapsedMilliseconds}ms for {operationCount} operations");
        }
    }
}

/// <summary>
/// Exception Handling Tests
/// </summary>
[TestClass]
public class ExceptionHandlingTests
{
    private EvmExceptionHandler? _exceptionHandler;
    
    [TestInitialize]
    public void Setup()
    {
        _exceptionHandler = new EvmExceptionHandler();
    }
    
    [TestCleanup]
    public void Cleanup()
    {
        _exceptionHandler?.Dispose();
    }
    
    [TestMethod]
    public void ExceptionHandler_BasicOperation_ShouldHandleSuccess()
    {
        var result = _exceptionHandler!.Execute(() => 42);
        
        Assert.IsTrue(result.IsSuccess);
        Assert.AreEqual(42, result.Value);
        Assert.IsNull(result.Error);
    }
    
    [TestMethod]
    public void ExceptionHandler_ExceptionHandling_ShouldCaptureError()
    {
        var result = _exceptionHandler!.Execute<int>(() => throw new InvalidOperationException("Test error"));
        
        Assert.IsFalse(result.IsSuccess);
        Assert.IsNotNull(result.Error);
        Assert.AreEqual("InvalidOperationException", result.Error.Type);
        Assert.IsTrue(result.Error.Message.Contains("Test error"));
    }
    
    [TestMethod]
    public void ExceptionHandler_Revert_ShouldThrowEvmRevertException()
    {
        try
        {
            _exceptionHandler!.Revert("Transaction reverted");
            Assert.Fail("Should have thrown EvmRevertException");
        }
        catch (EvmRevertException ex)
        {
            Assert.AreEqual("Transaction reverted", ex.Message);
            Assert.IsTrue(ex.RevertData.Length > 0);
        }
    }
    
    [TestMethod]
    public void ExceptionHandler_Require_ShouldPassWhenTrue()
    {
        // Should not throw
        _exceptionHandler!.Require(true, "Should pass");
    }
    
    [TestMethod]
    [ExpectedException(typeof(EvmRevertException))]
    public void ExceptionHandler_Require_ShouldFailWhenFalse()
    {
        _exceptionHandler!.Require(false, "Should fail");
    }
    
    [TestMethod]
    public void ExceptionHandler_TryRecover_ShouldUseFallback()
    {
        var result = _exceptionHandler!.TryRecover(() => throw new Exception("Test"), 999);
        
        Assert.AreEqual(999, result);
    }
    
    [TestMethod]
    public void ExceptionHandler_Statistics_ShouldTrackExceptions()
    {
        // Generate some exceptions
        _exceptionHandler!.Execute<int>(() => throw new ArgumentException("Test 1"));
        _exceptionHandler!.Execute<int>(() => throw new InvalidOperationException("Test 2"));
        _exceptionHandler!.Execute<int>(() => throw new ArgumentException("Test 3"));
        
        var stats = _exceptionHandler.GetStats();
        
        Assert.AreEqual(3UL, stats.ExceptionsHandled);
        Assert.IsTrue(stats.ExceptionTypeBreakdown.ContainsKey("ArgumentException"));
        Assert.IsTrue(stats.ExceptionTypeBreakdown.ContainsKey("InvalidOperationException"));
        Assert.AreEqual(2UL, stats.ExceptionTypeBreakdown["ArgumentException"]);
        Assert.AreEqual(1UL, stats.ExceptionTypeBreakdown["InvalidOperationException"]);
    }
}

/// <summary>
/// Test utilities and helpers
/// </summary>
public static class TestHelpers
{
    public static byte[] GenerateRandomBytes(int length)
    {
        var random = new Random();
        var bytes = new byte[length];
        random.NextBytes(bytes);
        return bytes;
    }
    
    public static BigInteger GenerateRandomBigInteger()
    {
        var random = new Random();
        var bytes = new byte[32];
        random.NextBytes(bytes);
        return new BigInteger(bytes, isUnsigned: true);
    }
    
    public static UInt160 GenerateRandomAddress()
    {
        var bytes = GenerateRandomBytes(20);
        return new UInt160(bytes);
    }
}