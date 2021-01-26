// SPDX-License-Identifier: MIT
// Copyright wtfsckgh@gmail.com
// Copyright iced contributors

#if GAS || INTEL || MASM || NASM || FAST_FMT
using Iced.Intel;

namespace Iced.UnitTests.Intel.FormatterTests {
	public readonly struct SymbolResolverTestCase {
		internal readonly int Bitness;
		internal readonly string HexBytes;
		internal readonly Code Code;
		internal readonly (OptionsProps property, object value)[] Options;
		internal readonly SymbolResultTestCase[] SymbolResults;
		internal SymbolResolverTestCase(int bitness, string hexBytes, Code code, (OptionsProps property, object value)[] options, SymbolResultTestCase[] symbolResults) {
			Bitness = bitness;
			HexBytes = hexBytes;
			Code = code;
			Options = options;
			SymbolResults = symbolResults;
		}
	}

	readonly struct SymbolResultTestCase {
		public readonly ulong Address;
		public readonly ulong SymbolAddress;
		public readonly int AddressSize;
		public readonly SymbolFlags Flags;
		public readonly MemorySize? MemorySize;
		public readonly string[] SymbolParts;
		public SymbolResultTestCase(ulong address, ulong symbolAddress, int addressSize, SymbolFlags flags, MemorySize? memorySize, string[] symbolParts) {
			Address = address;
			SymbolAddress = symbolAddress;
			AddressSize = addressSize;
			Flags = flags;
			MemorySize = memorySize;
			SymbolParts = symbolParts;
		}
	}
}
#endif
