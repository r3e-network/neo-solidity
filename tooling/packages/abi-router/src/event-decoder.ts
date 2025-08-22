import { EventFragment, LogDescription, Interface } from "ethers";
import Debug from "debug";

const debug = Debug("neo-solidity:event-decoder");

/**
 * Event decoder for Neo events to Ethereum format
 */
export class EventDecoder {

  /**
   * Decode Neo event to Ethereum log format
   */
  decodeEvent(neoEvent: any, eventFragment: EventFragment): LogDescription {
    debug(`Decoding event ${eventFragment.name}`);

    try {
      // Create interface for this event
      const iface = new Interface([eventFragment]);

      // Convert Neo event data to Ethereum log format
      const log = {
        topics: this.createTopicsFromNeoEvent(neoEvent, eventFragment),
        data: this.createDataFromNeoEvent(neoEvent, eventFragment),
        address: neoEvent.contract,
        blockNumber: neoEvent.blockNumber || 0,
        transactionHash: neoEvent.transactionHash || "0x0000000000000000000000000000000000000000000000000000000000000000",
        logIndex: 0
      };

      // Parse the log using ethers interface
      return iface.parseLog(log)!;
    } catch (error) {
      debug(`Event decoding failed: ${error}`);
      throw error;
    }
  }

  /**
   * Decode multiple events
   */
  decodeEvents(neoEvents: any[], eventFragment: EventFragment): LogDescription[] {
    return neoEvents.map(event => this.decodeEvent(event, eventFragment));
  }

  // Private methods

  private createTopicsFromNeoEvent(neoEvent: any, eventFragment: EventFragment): string[] {
    const topics: string[] = [];

    // First topic is always the event signature hash
    topics.push(this.getEventSignatureHash(eventFragment));

    // Add indexed parameter topics
    const indexedParams = eventFragment.inputs.filter(input => input.indexed);
    
    if (neoEvent.state && indexedParams.length > 0) {
      for (let i = 0; i < Math.min(indexedParams.length, neoEvent.state.length); i++) {
        const param = indexedParams[i];
        const value = neoEvent.state[i];
        
        topics.push(this.encodeIndexedParameter(value, param.type));
      }
    }

    return topics;
  }

  private createDataFromNeoEvent(neoEvent: any, eventFragment: EventFragment): string {
    // Encode non-indexed parameters as data
    const nonIndexedParams = eventFragment.inputs.filter(input => !input.indexed);
    
    if (!neoEvent.state || nonIndexedParams.length === 0) {
      return "0x";
    }

    // Skip indexed parameters to get non-indexed ones
    const indexedCount = eventFragment.inputs.filter(input => input.indexed).length;
    const nonIndexedValues = neoEvent.state.slice(indexedCount);

    return this.encodeEventData(nonIndexedValues, nonIndexedParams);
  }

  private getEventSignatureHash(eventFragment: EventFragment): string {
    // Create event signature
    const signature = `${eventFragment.name}(${eventFragment.inputs.map(input => input.type).join(',')})`;
    
    // This would normally use Keccak256, for now return mock hash
    return "0x" + signature.split('').map(c => 
      c.charCodeAt(0).toString(16)
    ).join('').padStart(64, '0').slice(0, 64);
  }

  private encodeIndexedParameter(value: any, type: string): string {
    // Encode indexed parameter as 32-byte topic
    switch (type) {
      case 'address':
        return this.padHex(value, 64);
      
      case 'bool':
        return this.padHex(value ? '01' : '00', 64);
      
      default:
        if (type.startsWith('uint') || type.startsWith('int')) {
          const hexValue = BigInt(value || 0).toString(16);
          return this.padHex(hexValue, 64);
        }
        
        if (type.startsWith('bytes')) {
          return this.padHex(value || '', 64);
        }
        
        // For other types, hash the value (simplified)
        return this.hashValue(String(value));
    }
  }

  private encodeEventData(values: any[], paramTypes: any[]): string {
    let encoded = '';
    
    for (let i = 0; i < values.length; i++) {
      const value = values[i];
      const param = paramTypes[i];
      
      if (!param) continue;
      
      encoded += this.encodeParameter(value, param.type);
    }
    
    return '0x' + encoded;
  }

  private encodeParameter(value: any, type: string): string {
    switch (type) {
      case 'bool':
        return this.padHex(value ? '01' : '00', 64);
      
      case 'address':
        return this.padHex(value, 64);
      
      case 'string':
        // For dynamic types, this would be more complex
        // Simplified encoding for now
        const hexString = Buffer.from(String(value), 'utf8').toString('hex');
        return this.padHex(hexString, 64);
      
      default:
        if (type.startsWith('uint') || type.startsWith('int')) {
          const hexValue = BigInt(value || 0).toString(16);
          return this.padHex(hexValue, 64);
        }
        
        if (type.startsWith('bytes')) {
          return this.padHex(value || '', 64);
        }
        
        // Default to string encoding
        const defaultHex = Buffer.from(String(value), 'utf8').toString('hex');
        return this.padHex(defaultHex, 64);
    }
  }

  private padHex(hex: string, length: number): string {
    const cleaned = hex.startsWith('0x') ? hex.slice(2) : hex;
    return cleaned.padStart(length, '0');
  }

  private hashValue(value: string): string {
    // This would normally use Keccak256
    // For now, return a mock hash based on the value
    return "0x" + value.split('').map(c => 
      c.charCodeAt(0).toString(16)
    ).join('').padStart(64, '0').slice(0, 64);
  }
}