package main

import "crypto/sha256"

// computeInteropID returns the first four bytes of the SHA-256 hash of the
// provided syscall name, matching Neo N3 interop ID encoding.
func computeInteropID(method string) []byte {
	sum := sha256.Sum256([]byte(method))
	id := make([]byte, 4)
	copy(id, sum[:4])
	return id
}
