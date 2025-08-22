using System.Numerics;
using System.Text;
using FluentAssertions;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Services;
using Neo.Sol.Runtime.Storage;
using NUnit.Framework;
using Moq;

namespace Neo.Sol.Runtime.Tests.Storage;

[TestFixture]
public class StorageManagerTests
{
    private StorageManager _storageManager = null!;
    private Mock<StorageContext> _mockStorageContext = null!;
    
    [SetUp]
    public void SetUp()
    {
        _mockStorageContext = new Mock<StorageContext>();
        _storageManager = new StorageManager(_mockStorageContext.Object);
    }
    
    [TearDown]
    public void TearDown()
    {
        _storageManager.ClearCache();
    }
    
    [Test]
    public void Constructor_ShouldInitializeWithContext()
    {
        // Arrange & Act
        var manager = new StorageManager(_mockStorageContext.Object);
        
        // Assert
        manager.Should().NotBeNull();
        var stats = manager.GetStats();
        stats.CacheSize.Should().Be(0);
    }
    
    [Test]
    public void Store_BigInteger_ShouldSerializeAndStore()
    {
        // Arrange
        var key = CreateNeoVMByteString("balance_key");
        var value = new BigInteger(1000000);
        
        // Act
        _storageManager.Store(key, value);
        
        // Assert
        var stored = _storageManager.Load(key);
        stored.Should().BeOfType<BigInteger>().Which.Should().Be(value);
    }
    
    [Test]
    public void Store_String_ShouldSerializeAndStore()
    {
        // Arrange
        var key = CreateNeoVMByteString("name_key");
        var value = "TestToken";
        
        // Act
        _storageManager.Store(key, value);
        
        // Assert
        var stored = _storageManager.Load(key);
        stored.Should().BeOfType<string>().Which.Should().Be(value);
    }
    
    [Test]
    public void Store_ByteArray_ShouldStoreDirectly()
    {
        // Arrange
        var key = CreateNeoVMByteString("data_key");
        var value = new byte[] { 0x01, 0x02, 0x03, 0x04 };
        
        // Act
        _storageManager.Store(key, value);
        
        // Assert
        var stored = _storageManager.Load(key);
        stored.Should().BeOfType<byte[]>().Which.Should().BeEquivalentTo(value);
    }
    
    [Test]
    public void Store_UInt160_ShouldSerializeAndStore()
    {
        // Arrange
        var key = CreateNeoVMByteString("address_key");
        var value = new UInt160(new byte[20] { 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A,
                                              0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14 });
        
        // Act
        _storageManager.Store(key, value);
        
        // Assert
        var stored = _storageManager.Load(key);
        stored.Should().BeOfType<UInt160>().Which.Should().Be(value);
    }
    
    [Test]
    public void Load_NonExistentKey_ShouldReturnNull()
    {
        // Arrange
        var key = CreateNeoVMByteString("nonexistent");
        
        // Act
        var result = _storageManager.Load(key);
        
        // Assert
        result.Should().BeNull();
    }
    
    [Test]
    public void Delete_ShouldRemoveValue()
    {
        // Arrange
        var key = CreateNeoVMByteString("to_delete");
        var value = "test_value";
        _storageManager.Store(key, value);
        
        // Act
        var deleted = _storageManager.Delete(key);
        
        // Assert
        deleted.Should().BeTrue();
        var result = _storageManager.Load(key);
        result.Should().BeNull();
    }
    
    [Test]
    public void Delete_NonExistentKey_ShouldReturnFalse()
    {
        // Arrange
        var key = CreateNeoVMByteString("nonexistent");
        
        // Act
        var deleted = _storageManager.Delete(key);
        
        // Assert
        deleted.Should().BeFalse();
    }
    
    [Test]
    public void Exists_ShouldReturnCorrectValue()
    {
        // Arrange
        var key1 = CreateNeoVMByteString("exists");
        var key2 = CreateNeoVMByteString("not_exists");
        _storageManager.Store(key1, "value");
        
        // Act & Assert
        _storageManager.Exists(key1).Should().BeTrue();
        _storageManager.Exists(key2).Should().BeFalse();
    }
    
    [Test]
    public void StoreMapping_ShouldHandleComplexKeys()
    {
        // Arrange
        var account = new UInt160(new byte[20] { 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A,
                                                 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14 });
        var spender = new UInt160(new byte[20] { 0x14, 0x13, 0x12, 0x11, 0x10, 0x0F, 0x0E, 0x0D, 0x0C, 0x0B,
                                                 0x0A, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01 });
        var allowance = new BigInteger(5000);
        
        // Act
        _storageManager.StoreMapping(account, spender, allowance);
        
        // Assert
        var stored = _storageManager.LoadMapping<BigInteger>(account, spender);
        stored.Should().Be(allowance);
    }
    
    [Test]
    public void LoadMapping_NonExistent_ShouldReturnDefault()
    {
        // Arrange
        var account = new UInt160(new byte[20]);
        var spender = new UInt160(new byte[20]);
        
        // Act
        var result = _storageManager.LoadMapping<BigInteger>(account, spender);
        
        // Assert
        result.Should().Be(default(BigInteger));
    }
    
    [Test]
    public void StoreArray_ShouldHandleArrayIndices()
    {
        // Arrange
        var baseKey = CreateNeoVMByteString("array");
        var values = new[] { "item0", "item1", "item2" };
        
        // Act
        for (int i = 0; i < values.Length; i++)
        {
            _storageManager.StoreArray(baseKey, i, values[i]);
        }
        
        // Assert
        for (int i = 0; i < values.Length; i++)
        {
            var stored = _storageManager.LoadArray<string>(baseKey, i);
            stored.Should().Be(values[i]);
        }
    }
    
    [Test]
    public void GetArrayLength_ShouldReturnCorrectLength()
    {
        // Arrange
        var baseKey = CreateNeoVMByteString("sized_array");
        var length = 10;
        
        // Act
        _storageManager.SetArrayLength(baseKey, length);
        
        // Assert
        var storedLength = _storageManager.GetArrayLength(baseKey);
        storedLength.Should().Be(length);
    }
    
    [Test]
    public void CacheOperations_ShouldImprovePerformance()
    {
        // Arrange
        var key = CreateNeoVMByteString("cached_value");
        var value = "test_cache";
        
        // Act
        _storageManager.Store(key, value);
        
        // First load should cache
        var first = _storageManager.Load(key);
        
        // Second load should use cache
        var second = _storageManager.Load(key);
        
        // Assert
        first.Should().Be(value);
        second.Should().Be(value);
        
        var stats = _storageManager.GetStats();
        stats.CacheSize.Should().BeGreaterThan(0);
        stats.CacheHits.Should().BeGreaterThan(0);
    }
    
    [Test]
    public void ClearCache_ShouldRemoveAllCachedItems()
    {
        // Arrange
        var key = CreateNeoVMByteString("cached_value");
        var value = "test_cache";
        _storageManager.Store(key, value);
        _storageManager.Load(key); // Cache the item
        
        // Act
        _storageManager.ClearCache();
        
        // Assert
        var stats = _storageManager.GetStats();
        stats.CacheSize.Should().Be(0);
    }
    
    [Test]
    public void StorageCostCalculation_ShouldAccountForDataSize()
    {
        // Arrange
        var key = CreateNeoVMByteString("cost_test");
        var smallValue = "small";
        var largeValue = new string('x', 1000);
        
        // Act
        var smallCost = _storageManager.CalculateStorageCost(key, smallValue);
        var largeCost = _storageManager.CalculateStorageCost(key, largeValue);
        
        // Assert
        largeCost.Should().BeGreaterThan(smallCost);
    }
    
    [Test]
    public void BulkOperations_ShouldHandleMultipleItems()
    {
        // Arrange
        var items = new Dictionary<byte[], object>
        {
            { Encoding.UTF8.GetBytes("key1"), "value1" },
            { Encoding.UTF8.GetBytes("key2"), new BigInteger(123) },
            { Encoding.UTF8.GetBytes("key3"), new byte[] { 0x01, 0x02, 0x03 } }
        };
        
        // Act
        _storageManager.BulkStore(items);
        
        // Assert
        foreach (var item in items)
        {
            var key = CreateNeoVMByteString(item.Key);
            var stored = _storageManager.Load(key);
            stored.Should().Be(item.Value);
        }
    }
    
    [Test]
    public void Iterator_ShouldAllowEnumeratingKeys()
    {
        // Arrange
        var prefix = "prefix_";
        var keys = new[] { "prefix_1", "prefix_2", "prefix_3", "other_key" };
        
        foreach (var key in keys)
        {
            _storageManager.Store(CreateNeoVMByteString(key), $"value_{key}");
        }
        
        // Act
        var prefixedKeys = _storageManager.FindKeys(Encoding.UTF8.GetBytes(prefix));
        
        // Assert
        prefixedKeys.Should().HaveCount(3);
        foreach (var key in prefixedKeys)
        {
            Encoding.UTF8.GetString(key.ToArray()).Should().StartWith(prefix);
        }
    }
    
    [Test]
    public void StoreWithTimestamp_ShouldRecordTimestamp()
    {
        // Arrange
        var key = CreateNeoVMByteString("timestamped");
        var value = "test_value";
        
        // Act
        _storageManager.StoreWithTimestamp(key, value);
        
        // Assert
        var metadata = _storageManager.GetKeyMetadata(key);
        metadata.Should().NotBeNull();
        metadata!.CreatedAt.Should().BeCloseTo(DateTime.UtcNow, TimeSpan.FromSeconds(1));
        metadata.LastModified.Should().BeCloseTo(DateTime.UtcNow, TimeSpan.FromSeconds(1));
    }
    
    [Test]
    public void GetStats_ShouldReturnComprehensiveStatistics()
    {
        // Arrange
        for (int i = 0; i < 5; i++)
        {
            var key = CreateNeoVMByteString($"stats_key_{i}");
            _storageManager.Store(key, $"value_{i}");
            _storageManager.Load(key); // Generate cache hit
        }
        
        // Act
        var stats = _storageManager.GetStats();
        
        // Assert
        stats.TotalKeys.Should().BeGreaterOrEqualTo(5);
        stats.CacheSize.Should().BeGreaterOrEqualTo(5);
        stats.CacheHits.Should().BeGreaterOrEqualTo(5);
        stats.TotalStorageBytes.Should().BeGreaterThan(0);
        stats.CacheHitRatio.Should().BeGreaterThan(0);
    }
    
    [Test]
    public void ConcurrentAccess_ShouldBThreadSafe()
    {
        // Arrange
        var tasks = new List<Task>();
        var keyCount = 100;
        
        // Act - Multiple threads storing different keys
        for (int i = 0; i < keyCount; i++)
        {
            var index = i;
            tasks.Add(Task.Run(() =>
            {
                var key = CreateNeoVMByteString($"concurrent_{index}");
                var value = $"value_{index}";
                _storageManager.Store(key, value);
            }));
        }
        
        Task.WaitAll(tasks.ToArray());
        
        // Assert - All values should be stored correctly
        for (int i = 0; i < keyCount; i++)
        {
            var key = CreateNeoVMByteString($"concurrent_{i}");
            var stored = _storageManager.Load(key);
            stored.Should().Be($"value_{i}");
        }
        
        var stats = _storageManager.GetStats();
        stats.TotalKeys.Should().BeGreaterOrEqualTo(keyCount);
    }
    
    [Test]
    public void SerializationRoundTrip_ShouldPreserveComplexObjects()
    {
        // Arrange
        var key = CreateNeoVMByteString("complex_object");
        var originalData = new Dictionary<string, object>
        {
            { "number", new BigInteger(123456789) },
            { "text", "Hello, World!" },
            { "bytes", new byte[] { 0xDE, 0xAD, 0xBE, 0xEF } },
            { "boolean", true }
        };
        
        // Act
        _storageManager.Store(key, originalData);
        var retrieved = _storageManager.Load(key);
        
        // Assert
        retrieved.Should().NotBeNull();
        retrieved.Should().BeOfType<Dictionary<string, object>>();
        
        var dict = (Dictionary<string, object>)retrieved!;
        dict["number"].Should().Be(originalData["number"]);
        dict["text"].Should().Be(originalData["text"]);
        dict["bytes"].Should().BeEquivalentTo(originalData["bytes"]);
        dict["boolean"].Should().Be(originalData["boolean"]);
    }
    
    // Helper method
    private static NeoVMByteString CreateNeoVMByteString(string value)
    {
        return new NeoVMByteString { Value = Encoding.UTF8.GetBytes(value) };
    }
    
    private static NeoVMByteString CreateNeoVMByteString(byte[] value)
    {
        return new NeoVMByteString { Value = value };
    }
}

// Mock types for testing
public class NeoVMByteString
{
    public byte[] Value { get; set; } = Array.Empty<byte>();
    
    public byte[] ToArray() => Value;
}