using System.Numerics;
using System.Text;
using FluentAssertions;
using Neo.SmartContract.Framework;
using Neo.Sol.Runtime.Events;
using NUnit.Framework;

namespace Neo.Sol.Runtime.Tests.Events;

[TestFixture]
public class EventManagerTests
{
    private EventManager _eventManager = null!;
    private UInt160 _contractAddress = null!;
    
    [SetUp]
    public void SetUp()
    {
        _contractAddress = new UInt160(new byte[20] 
        { 
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A,
            0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14 
        });
        _eventManager = new EventManager(_contractAddress);
    }
    
    [Test]
    public void Constructor_ShouldInitializeWithContractAddress()
    {
        // Arrange & Act
        var manager = new EventManager(_contractAddress);
        
        // Assert
        manager.Should().NotBeNull();
        manager.ContractAddress.Should().Be(_contractAddress);
    }
    
    [Test]
    public void Log0_ShouldEmitEventWithNoTopics()
    {
        // Arrange
        var data = Encoding.UTF8.GetBytes("test data");
        
        // Act
        var eventId = _eventManager.Log0(data);
        
        // Assert
        eventId.Should().NotBeEmpty();
        var events = _eventManager.GetEmittedEvents();
        events.Should().HaveCount(1);
        
        var emittedEvent = events.First();
        emittedEvent.Topics.Should().HaveCount(0);
        emittedEvent.Data.Should().BeEquivalentTo(data);
        emittedEvent.ContractAddress.Should().Be(_contractAddress);
    }
    
    [Test]
    public void Log1_ShouldEmitEventWithOneTopic()
    {
        // Arrange
        var topic1 = new UInt256(new byte[32] { 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 
                                                0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10,
                                                0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
                                                0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20 });
        var data = Encoding.UTF8.GetBytes("log1 data");
        
        // Act
        var eventId = _eventManager.Log1(data, topic1);
        
        // Assert
        eventId.Should().NotBeEmpty();
        var events = _eventManager.GetEmittedEvents();
        events.Should().HaveCount(1);
        
        var emittedEvent = events.First();
        emittedEvent.Topics.Should().HaveCount(1);
        emittedEvent.Topics[0].Should().Be(topic1);
        emittedEvent.Data.Should().BeEquivalentTo(data);
    }
    
    [Test]
    public void Log2_ShouldEmitEventWithTwoTopics()
    {
        // Arrange
        var topic1 = new UInt256(new byte[32]);
        var topic2 = new UInt256(new byte[32]);
        topic2.ToArray()[31] = 0xFF;
        var data = Encoding.UTF8.GetBytes("log2 data");
        
        // Act
        var eventId = _eventManager.Log2(data, topic1, topic2);
        
        // Assert
        eventId.Should().NotBeEmpty();
        var events = _eventManager.GetEmittedEvents();
        events.Should().HaveCount(1);
        
        var emittedEvent = events.First();
        emittedEvent.Topics.Should().HaveCount(2);
        emittedEvent.Topics[0].Should().Be(topic1);
        emittedEvent.Topics[1].Should().Be(topic2);
    }
    
    [Test]
    public void Log3_ShouldEmitEventWithThreeTopics()
    {
        // Arrange
        var topic1 = new UInt256(new byte[32]);
        var topic2 = new UInt256(new byte[32]);
        var topic3 = new UInt256(new byte[32]);
        topic3.ToArray()[0] = 0xAA;
        var data = Encoding.UTF8.GetBytes("log3 data");
        
        // Act
        var eventId = _eventManager.Log3(data, topic1, topic2, topic3);
        
        // Assert
        eventId.Should().NotBeEmpty();
        var events = _eventManager.GetEmittedEvents();
        events.Should().HaveCount(1);
        
        var emittedEvent = events.First();
        emittedEvent.Topics.Should().HaveCount(3);
        emittedEvent.Topics[2].Should().Be(topic3);
    }
    
    [Test]
    public void Log4_ShouldEmitEventWithFourTopics()
    {
        // Arrange
        var topics = new[]
        {
            new UInt256(new byte[32]),
            new UInt256(new byte[32]),
            new UInt256(new byte[32]),
            new UInt256(new byte[32])
        };
        topics[3].ToArray()[15] = 0xCC;
        var data = Encoding.UTF8.GetBytes("log4 data");
        
        // Act
        var eventId = _eventManager.Log4(data, topics[0], topics[1], topics[2], topics[3]);
        
        // Assert
        eventId.Should().NotBeEmpty();
        var events = _eventManager.GetEmittedEvents();
        events.Should().HaveCount(1);
        
        var emittedEvent = events.First();
        emittedEvent.Topics.Should().HaveCount(4);
        for (int i = 0; i < 4; i++)
        {
            emittedEvent.Topics[i].Should().Be(topics[i]);
        }
    }
    
    [Test]
    public void EmitEvent_WithSignature_ShouldCalculateEventHash()
    {
        // Arrange
        var signature = "Transfer(address,address,uint256)";
        var from = new UInt160(new byte[20]);
        var to = new UInt160(new byte[20]);
        to.ToArray()[19] = 0xFF;
        var amount = new BigInteger(1000);
        
        // Act
        var eventId = _eventManager.EmitEvent(signature, from, to, amount);
        
        // Assert
        eventId.Should().NotBeEmpty();
        var events = _eventManager.GetEmittedEvents();
        events.Should().HaveCount(1);
        
        var emittedEvent = events.First();
        emittedEvent.Topics.Should().HaveCount(3); // event hash + 2 indexed parameters
        emittedEvent.Signature.Should().Be(signature);
        
        // First topic should be the keccak256 hash of the signature
        var expectedHash = _eventManager.CalculateEventHash(signature);
        emittedEvent.Topics[0].Should().Be(expectedHash);
    }
    
    [Test]
    public void EmitTransferEvent_ShouldUseStandardSignature()
    {
        // Arrange
        var from = new UInt160(new byte[20]);
        var to = new UInt160(new byte[20]);
        var value = new BigInteger(500);
        
        // Act
        var eventId = _eventManager.EmitTransferEvent(from, to, value);
        
        // Assert
        var events = _eventManager.GetEmittedEvents();
        events.Should().HaveCount(1);
        
        var emittedEvent = events.First();
        emittedEvent.Signature.Should().Be("Transfer(address,address,uint256)");
        emittedEvent.Topics.Should().HaveCount(3);
        
        // Verify the encoded data contains the value
        var encodedValue = _eventManager.EncodeEventData(value);
        emittedEvent.Data.Should().BeEquivalentTo(encodedValue);
    }
    
    [Test]
    public void EmitApprovalEvent_ShouldUseStandardSignature()
    {
        // Arrange
        var owner = new UInt160(new byte[20]);
        var spender = new UInt160(new byte[20]);
        var value = new BigInteger(1000);
        
        // Act
        var eventId = _eventManager.EmitApprovalEvent(owner, spender, value);
        
        // Assert
        var events = _eventManager.GetEmittedEvents();
        events.Should().HaveCount(1);
        
        var emittedEvent = events.First();
        emittedEvent.Signature.Should().Be("Approval(address,address,uint256)");
        emittedEvent.Topics.Should().HaveCount(3);
    }
    
    [Test]
    public void EmitCustomEvent_ShouldHandleComplexParameters()
    {
        // Arrange
        var signature = "ComplexEvent(uint256,string,bytes32,bool)";
        var number = new BigInteger(123456);
        var text = "Hello World";
        var hash = new UInt256(new byte[32]);
        var flag = true;
        
        // Act
        var eventId = _eventManager.EmitEvent(signature, number, text, hash, flag);
        
        // Assert
        var events = _eventManager.GetEmittedEvents();
        events.Should().HaveCount(1);
        
        var emittedEvent = events.First();
        emittedEvent.Signature.Should().Be(signature);
        emittedEvent.Topics.Should().HaveCount(1); // Only event hash, no indexed params in this case
    }
    
    [Test]
    public void GetEventsByTopic_ShouldFilterCorrectly()
    {
        // Arrange
        var topic = new UInt256(new byte[32]);
        topic.ToArray()[0] = 0x42;
        
        _eventManager.Log1(Encoding.UTF8.GetBytes("data1"), topic);
        _eventManager.Log1(Encoding.UTF8.GetBytes("data2"), new UInt256(new byte[32]));
        _eventManager.Log2(Encoding.UTF8.GetBytes("data3"), topic, new UInt256(new byte[32]));
        
        // Act
        var filteredEvents = _eventManager.GetEventsByTopic(topic);
        
        // Assert
        filteredEvents.Should().HaveCount(2);
        filteredEvents.All(e => e.Topics.Contains(topic)).Should().BeTrue();
    }
    
    [Test]
    public void GetEventsBySignature_ShouldFilterCorrectly()
    {
        // Arrange
        var transferSignature = "Transfer(address,address,uint256)";
        var approvalSignature = "Approval(address,address,uint256)";
        
        _eventManager.EmitTransferEvent(new UInt160(new byte[20]), new UInt160(new byte[20]), new BigInteger(100));
        _eventManager.EmitApprovalEvent(new UInt160(new byte[20]), new UInt160(new byte[20]), new BigInteger(200));
        _eventManager.EmitTransferEvent(new UInt160(new byte[20]), new UInt160(new byte[20]), new BigInteger(300));
        
        // Act
        var transferEvents = _eventManager.GetEventsBySignature(transferSignature);
        var approvalEvents = _eventManager.GetEventsBySignature(approvalSignature);
        
        // Assert
        transferEvents.Should().HaveCount(2);
        approvalEvents.Should().HaveCount(1);
        transferEvents.All(e => e.Signature == transferSignature).Should().BeTrue();
        approvalEvents.All(e => e.Signature == approvalSignature).Should().BeTrue();
    }
    
    [Test]
    public void GetEventsInRange_ShouldFilterByBlockRange()
    {
        // Arrange
        var startBlock = 100u;
        var endBlock = 200u;
        
        // Simulate events in different blocks
        _eventManager.Log0(Encoding.UTF8.GetBytes("early"), blockNumber: 50);
        _eventManager.Log0(Encoding.UTF8.GetBytes("in-range-1"), blockNumber: 150);
        _eventManager.Log0(Encoding.UTF8.GetBytes("in-range-2"), blockNumber: 180);
        _eventManager.Log0(Encoding.UTF8.GetBytes("late"), blockNumber: 300);
        
        // Act
        var eventsInRange = _eventManager.GetEventsInRange(startBlock, endBlock);
        
        // Assert
        eventsInRange.Should().HaveCount(2);
        eventsInRange.All(e => e.BlockNumber >= startBlock && e.BlockNumber <= endBlock).Should().BeTrue();
    }
    
    [Test]
    public void CalculateEventHash_ShouldBeConsistent()
    {
        // Arrange
        var signature = "Transfer(address,address,uint256)";
        
        // Act
        var hash1 = _eventManager.CalculateEventHash(signature);
        var hash2 = _eventManager.CalculateEventHash(signature);
        
        // Assert
        hash1.Should().Be(hash2);
        hash1.Should().NotBe(new UInt256(new byte[32])); // Should not be zero
    }
    
    [Test]
    public void EncodeEventData_ShouldHandleDifferentTypes()
    {
        // Arrange & Act
        var encodedInt = _eventManager.EncodeEventData(new BigInteger(123));
        var encodedString = _eventManager.EncodeEventData("test");
        var encodedBool = _eventManager.EncodeEventData(true);
        var encodedBytes = _eventManager.EncodeEventData(new byte[] { 0x01, 0x02, 0x03 });
        
        // Assert
        encodedInt.Should().NotBeEmpty();
        encodedString.Should().NotBeEmpty();
        encodedBool.Should().NotBeEmpty();
        encodedBytes.Should().NotBeEmpty();
        
        // Different types should produce different encodings
        encodedInt.Should().NotBeEquivalentTo(encodedString);
        encodedString.Should().NotBeEquivalentTo(encodedBool);
    }
    
    [Test]
    public void GetStats_ShouldReturnEventStatistics()
    {
        // Arrange
        _eventManager.Log0(Encoding.UTF8.GetBytes("event1"));
        _eventManager.Log1(Encoding.UTF8.GetBytes("event2"), new UInt256(new byte[32]));
        _eventManager.EmitTransferEvent(new UInt160(new byte[20]), new UInt160(new byte[20]), new BigInteger(100));
        
        // Act
        var stats = _eventManager.GetStats();
        
        // Assert
        stats.TotalEvents.Should().Be(3);
        stats.EventsByType.Should().ContainKey("Log0");
        stats.EventsByType.Should().ContainKey("Log1");
        stats.EventsByType.Should().ContainKey("Transfer");
        stats.TotalDataBytes.Should().BeGreaterThan(0);
    }
    
    [Test]
    public void ClearEvents_ShouldRemoveAllEvents()
    {
        // Arrange
        _eventManager.Log0(Encoding.UTF8.GetBytes("event1"));
        _eventManager.Log1(Encoding.UTF8.GetBytes("event2"), new UInt256(new byte[32]));
        var initialCount = _eventManager.GetEmittedEvents().Count;
        initialCount.Should().BeGreaterThan(0);
        
        // Act
        _eventManager.ClearEvents();
        
        // Assert
        var events = _eventManager.GetEmittedEvents();
        events.Should().BeEmpty();
        
        var stats = _eventManager.GetStats();
        stats.TotalEvents.Should().Be(0);
    }
    
    [Test]
    public void ConcurrentEventEmission_ShouldBeThreadSafe()
    {
        // Arrange
        var tasks = new List<Task>();
        var eventCount = 100;
        
        // Act
        for (int i = 0; i < eventCount; i++)
        {
            var index = i;
            tasks.Add(Task.Run(() =>
            {
                var data = Encoding.UTF8.GetBytes($"concurrent_event_{index}");
                _eventManager.Log0(data);
            }));
        }
        
        Task.WaitAll(tasks.ToArray());
        
        // Assert
        var events = _eventManager.GetEmittedEvents();
        events.Should().HaveCount(eventCount);
        
        // Each event should be unique
        var distinctEvents = events.DistinctBy(e => Encoding.UTF8.GetString(e.Data)).Count();
        distinctEvents.Should().Be(eventCount);
    }
    
    [Test]
    public void EventSerialization_ShouldPreserveData()
    {
        // Arrange
        var from = new UInt160(new byte[20]);
        from.ToArray()[0] = 0x12;
        var to = new UInt160(new byte[20]);
        to.ToArray()[19] = 0x34;
        var value = new BigInteger(123456789);
        
        // Act
        _eventManager.EmitTransferEvent(from, to, value);
        var events = _eventManager.GetEmittedEvents();
        var serialized = _eventManager.SerializeEvent(events.First());
        var deserialized = _eventManager.DeserializeEvent(serialized);
        
        // Assert
        deserialized.Should().NotBeNull();
        deserialized!.Signature.Should().Be("Transfer(address,address,uint256)");
        deserialized.Topics.Should().HaveCount(3);
        deserialized.ContractAddress.Should().Be(_contractAddress);
        deserialized.Data.Should().BeEquivalentTo(events.First().Data);
    }
    
    [Test]
    public void EventFiltering_WithMultipleCriteria_ShouldWork()
    {
        // Arrange
        var topic1 = new UInt256(new byte[32]);
        topic1.ToArray()[0] = 0x01;
        var topic2 = new UInt256(new byte[32]);
        topic2.ToArray()[0] = 0x02;
        
        _eventManager.Log2(Encoding.UTF8.GetBytes("match"), topic1, topic2, blockNumber: 100);
        _eventManager.Log2(Encoding.UTF8.GetBytes("no-match-topic"), topic1, new UInt256(new byte[32]), blockNumber: 100);
        _eventManager.Log2(Encoding.UTF8.GetBytes("no-match-block"), topic1, topic2, blockNumber: 200);
        
        // Act
        var filter = new EventFilter
        {
            Topics = new[] { topic1, topic2 },
            FromBlock = 50,
            ToBlock = 150,
            ContractAddress = _contractAddress
        };
        
        var filteredEvents = _eventManager.GetFilteredEvents(filter);
        
        // Assert
        filteredEvents.Should().HaveCount(1);
        filteredEvents.First().Data.Should().BeEquivalentTo(Encoding.UTF8.GetBytes("match"));
    }
}

// Supporting types for tests
public class EventFilter
{
    public UInt256[]? Topics { get; set; }
    public uint? FromBlock { get; set; }
    public uint? ToBlock { get; set; }
    public UInt160? ContractAddress { get; set; }
}