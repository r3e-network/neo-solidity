using System.Diagnostics;
using System.Numerics;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.Sol.Runtime;
using Neo.Sol.Runtime.Memory;
using Neo.Sol.Runtime.Storage;
using Neo.Sol.Runtime.ABI;
using Neo.Sol.Runtime.Crypto;
using Neo.Sol.Runtime.Events;
using Neo.Sol.Runtime.Exceptions;

namespace Neo.Sol.Runtime.Tests;

/// <summary>
/// Comprehensive performance benchmarks for the Neo-Sol Runtime
/// Tests performance characteristics, scalability, and optimization opportunities
/// </summary>
[TestClass]
public class PerformanceBenchmarks
{
    private const int WARMUP_ITERATIONS = 100;
    private const int BENCHMARK_ITERATIONS = 1000;
    private const int STRESS_ITERATIONS = 10000;
    
    /// <summary>
    /// Memory Manager Performance Benchmarks
    /// </summary>
    [TestClass]
    public class MemoryBenchmarks
    {
        private EvmMemoryManager? _memoryManager;
        
        [TestInitialize]
        public void Setup()
        {
            _memoryManager = new EvmMemoryManager();
            WarmupMemory();
        }
        
        [TestCleanup]
        public void Cleanup()
        {
            _memoryManager?.Dispose();
        }
        
        private void WarmupMemory()
        {
            for (int i = 0; i < WARMUP_ITERATIONS; i++)
            {
                _memoryManager!.Store((uint)i * 32, BigInteger.One);
            }
        }
        
        [TestMethod]
        public void Benchmark_MemoryStore_Sequential()
        {
            var results = new BenchmarkResult("Sequential Memory Store");
            
            var stopwatch = Stopwatch.StartNew();
            
            for (uint i = 0; i < BENCHMARK_ITERATIONS; i++)
            {
                _memoryManager!.Store(i * 32, new BigInteger(i));
            }
            
            stopwatch.Stop();
            results.RecordResult(BENCHMARK_ITERATIONS, stopwatch.ElapsedMilliseconds);
            
            Console.WriteLine(results.ToString());
            
            // Performance expectations
            Assert.IsTrue(results.OperationsPerSecond > 10000, "Sequential memory store should exceed 10k ops/sec");
        }
        
        [TestMethod]
        public void Benchmark_MemoryLoad_Sequential()
        {
            // Pre-populate memory
            for (uint i = 0; i < BENCHMARK_ITERATIONS; i++)
            {
                _memoryManager!.Store(i * 32, new BigInteger(i));
            }
            
            var results = new BenchmarkResult("Sequential Memory Load");
            var stopwatch = Stopwatch.StartNew();
            
            for (uint i = 0; i < BENCHMARK_ITERATIONS; i++)
            {
                var value = _memoryManager!.LoadBigInteger(i * 32);
            }
            
            stopwatch.Stop();
            results.RecordResult(BENCHMARK_ITERATIONS, stopwatch.ElapsedMilliseconds);
            
            Console.WriteLine(results.ToString());
            Assert.IsTrue(results.OperationsPerSecond > 50000, "Sequential memory load should exceed 50k ops/sec");
        }
        
        [TestMethod]
        public void Benchmark_MemoryStore_Random()
        {
            var random = new Random(12345); // Fixed seed for reproducibility
            var addresses = new uint[BENCHMARK_ITERATIONS];
            
            for (int i = 0; i < BENCHMARK_ITERATIONS; i++)
            {
                addresses[i] = (uint)random.Next(0, 100000) * 32;
            }
            
            var results = new BenchmarkResult("Random Memory Store");
            var stopwatch = Stopwatch.StartNew();
            
            for (int i = 0; i < BENCHMARK_ITERATIONS; i++)
            {
                _memoryManager!.Store(addresses[i], new BigInteger(i));
            }
            
            stopwatch.Stop();
            results.RecordResult(BENCHMARK_ITERATIONS, stopwatch.ElapsedMilliseconds);
            
            Console.WriteLine(results.ToString());
            Assert.IsTrue(results.OperationsPerSecond > 5000, "Random memory store should exceed 5k ops/sec");
        }
        
        [TestMethod]
        public void Benchmark_MemoryGarbageCollection()
        {
            // Allocate a lot of memory pages
            for (uint i = 0; i < 10000; i++)
            {
                _memoryManager!.Store(i * 4096, BigInteger.One); // One per page
            }
            
            var results = new BenchmarkResult("Memory Garbage Collection");
            var stopwatch = Stopwatch.StartNew();
            
            for (int i = 0; i < 10; i++)
            {
                _memoryManager!.ForceGarbageCollection();
            }
            
            stopwatch.Stop();
            results.RecordResult(10, stopwatch.ElapsedMilliseconds);
            
            Console.WriteLine(results.ToString());
            Assert.IsTrue(stopwatch.ElapsedMilliseconds < 1000, "GC should complete within 1 second");
        }
        
        [TestMethod]
        public void StressTest_MemoryManager_ConcurrentAccess()
        {
            const int threadCount = 8;
            const int operationsPerThread = STRESS_ITERATIONS / threadCount;
            var results = new BenchmarkResult("Concurrent Memory Access Stress Test");
            
            var tasks = new Task[threadCount];
            var stopwatch = Stopwatch.StartNew();
            
            for (int t = 0; t < threadCount; t++)
            {
                var threadId = t;
                tasks[t] = Task.Run(() =>
                {
                    for (int i = 0; i < operationsPerThread; i++)
                    {
                        var address = (uint)(threadId * operationsPerThread + i) * 32;
                        var value = new BigInteger(threadId * 1000000 + i);
                        
                        _memoryManager!.Store(address, value);
                        var retrieved = _memoryManager.LoadBigInteger(address);
                        
                        Assert.AreEqual(value, retrieved);
                    }
                });
            }
            
            Task.WaitAll(tasks);
            stopwatch.Stop();
            
            results.RecordResult(STRESS_ITERATIONS, stopwatch.ElapsedMilliseconds);
            Console.WriteLine(results.ToString());
            
            Assert.IsTrue(results.OperationsPerSecond > 1000, "Concurrent access should exceed 1k ops/sec");
        }
    }
    
    /// <summary>
    /// Storage Manager Performance Benchmarks
    /// </summary>
    [TestClass]
    public class StorageBenchmarks
    {
        private StorageManager? _storageManager;
        private Mock.StorageContext? _mockContext;
        
        [TestInitialize]
        public void Setup()
        {
            _mockContext = new Mock.StorageContext();
            _storageManager = new StorageManager(_mockContext);
            WarmupStorage();
        }
        
        [TestCleanup]
        public void Cleanup()
        {
            _storageManager?.Dispose();
        }
        
        private void WarmupStorage()
        {
            for (int i = 0; i < WARMUP_ITERATIONS; i++)
            {
                _storageManager!.Store(new BigInteger(i), new BigInteger(i));
            }
        }
        
        [TestMethod]
        public void Benchmark_StorageStore_Sequential()
        {
            var results = new BenchmarkResult("Sequential Storage Store");
            var stopwatch = Stopwatch.StartNew();
            
            for (int i = 0; i < BENCHMARK_ITERATIONS; i++)
            {
                _storageManager!.Store(new BigInteger(i), new BigInteger(i * 2));
            }
            
            stopwatch.Stop();
            results.RecordResult(BENCHMARK_ITERATIONS, stopwatch.ElapsedMilliseconds);
            
            Console.WriteLine(results.ToString());
            Assert.IsTrue(results.OperationsPerSecond > 1000, "Sequential storage store should exceed 1k ops/sec");
        }
        
        [TestMethod]
        public void Benchmark_StorageLoad_CacheHitRatio()
        {
            // Pre-populate storage
            var slots = new List<BigInteger>();
            for (int i = 0; i < 100; i++)
            {
                var slot = new BigInteger(i);
                _storageManager!.Store(slot, new BigInteger(i * 3));
                slots.Add(slot);
            }
            
            var results = new BenchmarkResult("Storage Load Cache Performance");
            var random = new Random(12345);
            var stopwatch = Stopwatch.StartNew();
            
            for (int i = 0; i < BENCHMARK_ITERATIONS; i++)
            {
                // 80% cache hits, 20% misses
                var slot = i % 5 == 0 
                    ? new BigInteger(i + 1000) // Cache miss
                    : slots[random.Next(slots.Count)]; // Cache hit
                    
                var value = _storageManager!.LoadBigInteger(slot);
            }
            
            stopwatch.Stop();
            results.RecordResult(BENCHMARK_ITERATIONS, stopwatch.ElapsedMilliseconds);
            
            Console.WriteLine(results.ToString());
            
            var stats = _storageManager!.GetStats();
            Console.WriteLine($"Cache Hit Ratio: {stats.CacheHitRatio:P2}");
            
            Assert.IsTrue(stats.CacheHitRatio > 0.7, "Cache hit ratio should exceed 70%");
        }
        
        [TestMethod]
        public void Benchmark_StorageBatchOperations()
        {
            var batchSize = 100;
            var updates = new Dictionary<BigInteger, byte[]>();
            
            for (int i = 0; i < batchSize; i++)
            {
                var slot = new BigInteger(i);
                var value = new byte[32];
                Array.Fill(value, (byte)(i % 256));
                updates[slot] = value;
            }
            
            var results = new BenchmarkResult("Storage Batch Operations");
            var iterations = BENCHMARK_ITERATIONS / batchSize;
            var stopwatch = Stopwatch.StartNew();
            
            for (int i = 0; i < iterations; i++)
            {
                _storageManager!.BatchStore(updates);
                var loaded = _storageManager.BatchLoad(updates.Keys);
            }
            
            stopwatch.Stop();
            results.RecordResult(iterations * batchSize * 2, stopwatch.ElapsedMilliseconds); // Store + Load
            
            Console.WriteLine(results.ToString());
            Assert.IsTrue(results.OperationsPerSecond > 500, "Batch operations should exceed 500 ops/sec");
        }
        
        [TestMethod]
        public void Benchmark_StorageSlotCalculation()
        {
            var results = new BenchmarkResult("Storage Slot Calculation");
            var stopwatch = Stopwatch.StartNew();
            
            for (int i = 0; i < BENCHMARK_ITERATIONS; i++)
            {
                var mappingSlot = StorageManager.CalculateMappingElementSlot(
                    new BigInteger(i % 10), 
                    new BigInteger(i));
                    
                var arraySlot = StorageManager.CalculateArrayElementSlot(
                    new BigInteger(i % 5), 
                    new BigInteger(i));
            }
            
            stopwatch.Stop();
            results.RecordResult(BENCHMARK_ITERATIONS * 2, stopwatch.ElapsedMilliseconds);
            
            Console.WriteLine(results.ToString());
            Assert.IsTrue(results.OperationsPerSecond > 100000, "Slot calculations should exceed 100k ops/sec");
        }
    }
    
    /// <summary>
    /// ABI Encoder/Decoder Performance Benchmarks
    /// </summary>
    [TestClass]
    public class AbiBenchmarks
    {
        [TestMethod]
        public void Benchmark_AbiEncoding_BasicTypes()
        {
            var results = new BenchmarkResult("ABI Encoding Basic Types");
            var stopwatch = Stopwatch.StartNew();
            
            for (int i = 0; i < BENCHMARK_ITERATIONS; i++)
            {
                var boolEncoded = AbiEncoder.EncodeUint256(i % 2);
                var uintEncoded = AbiEncoder.EncodeUint256(new BigInteger(i));
                var intEncoded = AbiEncoder.EncodeInt256(new BigInteger(-i));
                var addressEncoded = AbiEncoder.EncodeAddress(UInt160.Zero);
            }
            
            stopwatch.Stop();
            results.RecordResult(BENCHMARK_ITERATIONS * 4, stopwatch.ElapsedMilliseconds);
            
            Console.WriteLine(results.ToString());
            Assert.IsTrue(results.OperationsPerSecond > 50000, "ABI basic encoding should exceed 50k ops/sec");
        }
        
        [TestMethod]
        public void Benchmark_AbiEncoding_ComplexTypes()
        {
            var testString = "Hello, World! This is a test string for ABI encoding performance.";
            var testArray = new BigInteger[] { BigInteger.One, BigInteger.Parse("1000"), BigInteger.Parse("1000000") };
            
            var results = new BenchmarkResult("ABI Encoding Complex Types");
            var stopwatch = Stopwatch.StartNew();
            
            for (int i = 0; i < BENCHMARK_ITERATIONS; i++)
            {
                var parameters = new object[] { new BigInteger(i), testString, testArray };
                var encoded = AbiEncoder.EncodeParameters(parameters);
            }
            
            stopwatch.Stop();
            results.RecordResult(BENCHMARK_ITERATIONS, stopwatch.ElapsedMilliseconds);
            
            Console.WriteLine(results.ToString());
            Assert.IsTrue(results.OperationsPerSecond > 5000, "ABI complex encoding should exceed 5k ops/sec");
        }
        
        [TestMethod]
        public void Benchmark_FunctionSelectorCalculation()
        {
            var signatures = new[]
            {
                "transfer(address,uint256)",
                "approve(address,uint256)",
                "transferFrom(address,address,uint256)",
                "balanceOf(address)",
                "allowance(address,address)"
            };
            
            var results = new BenchmarkResult("Function Selector Calculation");
            var stopwatch = Stopwatch.StartNew();
            
            for (int i = 0; i < BENCHMARK_ITERATIONS; i++)
            {
                var signature = signatures[i % signatures.Length];
                var selector = AbiEncoder.CalculateFunctionSelector(signature);
            }
            
            stopwatch.Stop();
            results.RecordResult(BENCHMARK_ITERATIONS, stopwatch.ElapsedMilliseconds);
            
            Console.WriteLine(results.ToString());
            Assert.IsTrue(results.OperationsPerSecond > 20000, "Function selector calculation should exceed 20k ops/sec");
        }
        
        [TestMethod]
        public void Benchmark_AbiPackedEncoding()
        {
            var results = new BenchmarkResult("ABI Packed Encoding");
            var stopwatch = Stopwatch.StartNew();
            
            for (int i = 0; i < BENCHMARK_ITERATIONS; i++)
            {
                var values = new object[]
                {
                    (byte)(i % 256),
                    (ushort)(i % 65536),
                    (uint)i,
                    $"test_{i}"
                };
                
                var packed = AbiEncoder.EncodePacked(values);
            }
            
            stopwatch.Stop();
            results.RecordResult(BENCHMARK_ITERATIONS, stopwatch.ElapsedMilliseconds);
            
            Console.WriteLine(results.ToString());
            Assert.IsTrue(results.OperationsPerSecond > 10000, "Packed encoding should exceed 10k ops/sec");
        }
    }
    
    /// <summary>
    /// Cryptographic Functions Performance Benchmarks
    /// </summary>
    [TestClass]
    public class CryptoBenchmarks
    {
        [TestMethod]
        public void Benchmark_Keccak256_SmallInputs()
        {
            var testData = "Hello, World!"u8.ToArray();
            var results = new BenchmarkResult("Keccak256 Small Inputs");
            var stopwatch = Stopwatch.StartNew();
            
            for (int i = 0; i < BENCHMARK_ITERATIONS; i++)
            {
                var hash = CryptoLib.Keccak256(testData);
            }
            
            stopwatch.Stop();
            results.RecordResult(BENCHMARK_ITERATIONS, stopwatch.ElapsedMilliseconds);
            
            Console.WriteLine(results.ToString());
            Assert.IsTrue(results.OperationsPerSecond > 1000, "Keccak256 should exceed 1k ops/sec for small inputs");
        }
        
        [TestMethod]
        public void Benchmark_Keccak256_LargeInputs()
        {
            var testData = new byte[1024]; // 1KB
            Array.Fill(testData, (byte)0x42);
            
            var results = new BenchmarkResult("Keccak256 Large Inputs (1KB)");
            var stopwatch = Stopwatch.StartNew();
            
            for (int i = 0; i < BENCHMARK_ITERATIONS; i++)
            {
                var hash = CryptoLib.Keccak256(testData);
            }
            
            stopwatch.Stop();
            results.RecordResult(BENCHMARK_ITERATIONS, stopwatch.ElapsedMilliseconds);
            
            Console.WriteLine(results.ToString());
            Assert.IsTrue(results.OperationsPerSecond > 100, "Keccak256 should exceed 100 ops/sec for 1KB inputs");
        }
        
        [TestMethod]
        public void Benchmark_Sha256_Comparison()
        {
            var testData = "Hello, World!"u8.ToArray();
            var results = new BenchmarkResult("SHA256 vs Keccak256 Comparison");
            
            var stopwatch = Stopwatch.StartNew();
            for (int i = 0; i < BENCHMARK_ITERATIONS; i++)
            {
                var sha256Hash = CryptoLib.Sha256(testData);
            }
            var sha256Time = stopwatch.ElapsedMilliseconds;
            
            stopwatch.Restart();
            for (int i = 0; i < BENCHMARK_ITERATIONS; i++)
            {
                var keccakHash = CryptoLib.Keccak256(testData);
            }
            var keccakTime = stopwatch.ElapsedMilliseconds;
            
            Console.WriteLine($"SHA256: {BENCHMARK_ITERATIONS * 1000 / Math.Max(sha256Time, 1)} ops/sec");
            Console.WriteLine($"Keccak256: {BENCHMARK_ITERATIONS * 1000 / Math.Max(keccakTime, 1)} ops/sec");
            
            results.RecordResult(BENCHMARK_ITERATIONS * 2, sha256Time + keccakTime);
            Console.WriteLine(results.ToString());
        }
        
        [TestMethod]
        public void Benchmark_PublicKeyToAddress()
        {
            var publicKeys = new byte[10][];
            for (int i = 0; i < 10; i++)
            {
                publicKeys[i] = new byte[64];
                Array.Fill(publicKeys[i], (byte)(i * 25));
            }
            
            var results = new BenchmarkResult("Public Key to Address Conversion");
            var stopwatch = Stopwatch.StartNew();
            
            for (int i = 0; i < BENCHMARK_ITERATIONS; i++)
            {
                var publicKey = publicKeys[i % publicKeys.Length];
                var address = CryptoLib.PublicKeyToAddress(publicKey);
            }
            
            stopwatch.Stop();
            results.RecordResult(BENCHMARK_ITERATIONS, stopwatch.ElapsedMilliseconds);
            
            Console.WriteLine(results.ToString());
            Assert.IsTrue(results.OperationsPerSecond > 1000, "Public key to address should exceed 1k ops/sec");
        }
    }
    
    /// <summary>
    /// Exception Handling Performance Benchmarks
    /// </summary>
    [TestClass]
    public class ExceptionBenchmarks
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
        public void Benchmark_SuccessfulOperations()
        {
            var results = new BenchmarkResult("Successful Operations (No Exceptions)");
            var stopwatch = Stopwatch.StartNew();
            
            for (int i = 0; i < BENCHMARK_ITERATIONS; i++)
            {
                var result = _exceptionHandler!.Execute(() => i * 2);
            }
            
            stopwatch.Stop();
            results.RecordResult(BENCHMARK_ITERATIONS, stopwatch.ElapsedMilliseconds);
            
            Console.WriteLine(results.ToString());
            Assert.IsTrue(results.OperationsPerSecond > 50000, "Successful operations should exceed 50k ops/sec");
        }
        
        [TestMethod]
        public void Benchmark_ExceptionHandling()
        {
            var results = new BenchmarkResult("Exception Handling");
            var stopwatch = Stopwatch.StartNew();
            
            for (int i = 0; i < BENCHMARK_ITERATIONS; i++)
            {
                var result = _exceptionHandler!.Execute<int>(() => throw new ArgumentException($"Test {i}"));
            }
            
            stopwatch.Stop();
            results.RecordResult(BENCHMARK_ITERATIONS, stopwatch.ElapsedMilliseconds);
            
            Console.WriteLine(results.ToString());
            
            var stats = _exceptionHandler!.GetStats();
            Console.WriteLine($"Exceptions handled: {stats.ExceptionsHandled}");
            
            Assert.IsTrue(results.OperationsPerSecond > 1000, "Exception handling should exceed 1k ops/sec");
        }
        
        [TestMethod]
        public void Benchmark_TryRecover()
        {
            var results = new BenchmarkResult("Try Recover Operations");
            var stopwatch = Stopwatch.StartNew();
            
            for (int i = 0; i < BENCHMARK_ITERATIONS; i++)
            {
                var result = _exceptionHandler!.TryRecover(
                    () => throw new InvalidOperationException($"Test {i}"), 
                    -1);
            }
            
            stopwatch.Stop();
            results.RecordResult(BENCHMARK_ITERATIONS, stopwatch.ElapsedMilliseconds);
            
            Console.WriteLine(results.ToString());
            Assert.IsTrue(results.OperationsPerSecond > 5000, "Try recover should exceed 5k ops/sec");
        }
    }
    
    /// <summary>
    /// Integrated Runtime Performance Tests
    /// </summary>
    [TestClass]
    public class IntegratedBenchmarks
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
        public void Benchmark_ERC20_TransferSimulation()
        {
            // Simulate ERC20 transfer operations
            var results = new BenchmarkResult("ERC20 Transfer Simulation");
            var stopwatch = Stopwatch.StartNew();
            
            for (int i = 0; i < BENCHMARK_ITERATIONS; i++)
            {
                var from = TestHelpers.GenerateRandomAddress();
                var to = TestHelpers.GenerateRandomAddress();
                var amount = new BigInteger(i * 100);
                
                // Simulate balance checks and updates
                var fromBalanceSlot = StorageManager.CalculateMappingElementSlot(BigInteger.Zero, from.ToArray());
                var toBalanceSlot = StorageManager.CalculateMappingElementSlot(BigInteger.Zero, to.ToArray());
                
                var fromBalance = _runtime!.Storage.LoadBigInteger(fromBalanceSlot);
                var toBalance = _runtime.Storage.LoadBigInteger(toBalanceSlot);
                
                // Update balances
                _runtime.Storage.Store(fromBalanceSlot, fromBalance - amount);
                _runtime.Storage.Store(toBalanceSlot, toBalance + amount);
                
                // Emit transfer event
                StandardEvents.EmitTransfer(_runtime.Events, from, to, amount);
            }
            
            stopwatch.Stop();
            results.RecordResult(BENCHMARK_ITERATIONS, stopwatch.ElapsedMilliseconds);
            
            Console.WriteLine(results.ToString());
            Assert.IsTrue(results.OperationsPerSecond > 100, "ERC20 simulation should exceed 100 ops/sec");
        }
        
        [TestMethod]
        public void Benchmark_ComplexContractOperation()
        {
            // Simulate a complex contract operation involving multiple components
            var results = new BenchmarkResult("Complex Contract Operation");
            var stopwatch = Stopwatch.StartNew();
            
            for (int i = 0; i < BENCHMARK_ITERATIONS; i++)
            {
                // Use memory
                _runtime!.Memory.Store((uint)i * 32, new BigInteger(i));
                
                // Use storage
                _runtime.Storage.Store(new BigInteger(i), new BigInteger(i * 2));
                
                // Perform crypto operation
                var data = $"operation_{i}"u8.ToArray();
                var hash = _runtime.Keccak256(data);
                
                // Use ABI encoding
                var encoded = Evm.EncodeCall("complexOperation(uint256,bytes32)", i, hash);
                
                // Emit event
                _runtime.Events.Log1("ComplexOperation(uint256)", new BigInteger(i));
            }
            
            stopwatch.Stop();
            results.RecordResult(BENCHMARK_ITERATIONS, stopwatch.ElapsedMilliseconds);
            
            Console.WriteLine(results.ToString());
            
            var stats = _runtime!.GetStats();
            Console.WriteLine($"Memory utilization: {stats.MemoryStats.UtilizationRatio:P2}");
            Console.WriteLine($"Storage cache hit ratio: {stats.StorageStats.CacheHitRatio:P2}");
            
            Assert.IsTrue(results.OperationsPerSecond > 50, "Complex operations should exceed 50 ops/sec");
        }
        
        [TestMethod]
        public void StressTest_RuntimeUnderLoad()
        {
            const int stressIterations = STRESS_ITERATIONS / 10; // Reduce for integrated test
            var results = new BenchmarkResult("Runtime Stress Test");
            var stopwatch = Stopwatch.StartNew();
            
            var tasks = new Task[4]; // Use 4 concurrent tasks
            
            for (int t = 0; t < tasks.Length; t++)
            {
                var taskId = t;
                tasks[t] = Task.Run(() =>
                {
                    for (int i = 0; i < stressIterations / tasks.Length; i++)
                    {
                        var operationId = taskId * 1000000 + i;
                        
                        // Mix of operations
                        switch (i % 4)
                        {
                            case 0:
                                _runtime!.Memory.Store((uint)operationId * 32, new BigInteger(operationId));
                                break;
                            case 1:
                                _runtime!.Storage.Store(new BigInteger(operationId), new BigInteger(operationId));
                                break;
                            case 2:
                                var hash = _runtime!.Keccak256($"data_{operationId}"u8.ToArray());
                                break;
                            case 3:
                                _runtime!.Events.Log1("StressTest(uint256)", new BigInteger(operationId));
                                break;
                        }
                    }
                });
            }
            
            Task.WaitAll(tasks);
            stopwatch.Stop();
            
            results.RecordResult(stressIterations, stopwatch.ElapsedMilliseconds);
            Console.WriteLine(results.ToString());
            
            var stats = _runtime!.GetStats();
            Console.WriteLine($"Final Stats - Memory: {stats.MemoryStats.TotalSize} bytes, " +
                            $"Storage: {stats.StorageStats.ModifiedSlots} modified slots");
            
            Assert.IsTrue(stopwatch.ElapsedSeconds < 30, "Stress test should complete within 30 seconds");
        }
    }
}

/// <summary>
/// Benchmark result tracking and reporting
/// </summary>
public class BenchmarkResult
{
    public string TestName { get; }
    public long OperationsPerSecond { get; private set; }
    public long TotalOperations { get; private set; }
    public long ElapsedMilliseconds { get; private set; }
    public double AverageOperationTimeUs { get; private set; }
    
    public BenchmarkResult(string testName)
    {
        TestName = testName;
    }
    
    public void RecordResult(long operations, long elapsedMs)
    {
        TotalOperations = operations;
        ElapsedMilliseconds = Math.Max(elapsedMs, 1); // Prevent division by zero
        OperationsPerSecond = (operations * 1000) / ElapsedMilliseconds;
        AverageOperationTimeUs = (double)(ElapsedMilliseconds * 1000) / operations;
    }
    
    public override string ToString()
    {
        return $"{TestName}: {OperationsPerSecond:N0} ops/sec " +
               $"({TotalOperations:N0} ops in {ElapsedMilliseconds:N0}ms, " +
               $"avg: {AverageOperationTimeUs:F2}µs/op)";
    }
}

/// <summary>
/// Mock storage context for testing
/// </summary>
namespace Mock
{
    public class StorageContext
    {
        private readonly Dictionary<byte[], byte[]> _storage = new();
        
        public byte[]? Get(byte[] key)
        {
            return _storage.TryGetValue(key, out var value) ? value : null;
        }
        
        public void Put(byte[] key, byte[] value)
        {
            _storage[key] = value;
        }
        
        public void Delete(byte[] key)
        {
            _storage.Remove(key);
        }
    }
}

public static class TestHelpers
{
    private static readonly Random _random = new Random(12345); // Fixed seed for reproducibility
    
    public static byte[] GenerateRandomBytes(int length)
    {
        var bytes = new byte[length];
        _random.NextBytes(bytes);
        return bytes;
    }
    
    public static BigInteger GenerateRandomBigInteger()
    {
        var bytes = new byte[32];
        _random.NextBytes(bytes);
        return new BigInteger(bytes, isUnsigned: true);
    }
    
    public static UInt160 GenerateRandomAddress()
    {
        var bytes = GenerateRandomBytes(20);
        return new UInt160(bytes);
    }
}