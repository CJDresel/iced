// SPDX-License-Identifier: MIT
// Copyright wtfsckgh@gmail.com
// Copyright iced contributors

#if FAST_FMT
using System;

namespace Iced.Intel.FastFormatterInternal {
	static class MemorySizes {
		public static readonly string[] AllMemorySizes = GetMemorySizes();

		static string[] GetMemorySizes() {
			var memSizes = new string[IcedConstants.MemorySizeEnumCount];
#if HAS_SPAN
			System.ReadOnlySpan<byte> data =
#else
			byte[] data =
#endif
			new byte[IcedConstants.MemorySizeEnumCount] {
				// GENERATOR-BEGIN: MemorySizes
				// ⚠️This was generated by GENERATOR!🦹‍♂️
				0x00,
				0x01,
				0x0D,
				0x03,
				0x0B,
				0x0B,
				0x0E,
				0x0F,
				0x10,
				0x01,
				0x0D,
				0x03,
				0x0B,
				0x0E,
				0x0F,
				0x10,
				0x03,
				0x08,
				0x0C,
				0x0D,
				0x03,
				0x0B,
				0x03,
				0x0B,
				0x0B,
				0x09,
				0x08,
				0x08,
				0x0D,
				0x03,
				0x0B,
				0x0C,
				0x0E,
				0x0D,
				0x04,
				0x05,
				0x07,
				0x06,
				0x00,
				0x00,
				0x00,
				0x00,
				0x0C,
				0x10,
				0x00,
				0x0C,
				0x11,
				0x10,
				0x0D,
				0x0D,
				0x03,
				0x03,
				0x03,
				0x03,
				0x03,
				0x0B,
				0x0B,
				0x0B,
				0x0B,
				0x0B,
				0x0B,
				0x0B,
				0x0B,
				0x0E,
				0x0E,
				0x0E,
				0x0E,
				0x0E,
				0x0E,
				0x0E,
				0x0E,
				0x0E,
				0x0E,
				0x0E,
				0x0E,
				0x0E,
				0x0F,
				0x0F,
				0x0F,
				0x0F,
				0x0F,
				0x0F,
				0x0F,
				0x0F,
				0x0F,
				0x0F,
				0x0F,
				0x0F,
				0x0F,
				0x0F,
				0x0F,
				0x0F,
				0x10,
				0x10,
				0x10,
				0x10,
				0x10,
				0x10,
				0x10,
				0x10,
				0x10,
				0x10,
				0x10,
				0x10,
				0x10,
				0x02,
				0x02,
				0x02,
				0x02,
				0x02,
				0x0A,
				0x0A,
				0x0A,
				0x02,
				0x0A,
				0x02,
				0x02,
				0x0A,
				0x0A,
				0x0A,
				0x02,
				0x0A,
				0x02,
				0x02,
				0x0A,
				0x0A,
				0x0A,
				0x02,
				0x0A,
				0x02,
				0x02,
				0x02,
				0x0A,
				0x0A,
				0x0A,
				0x0A,
				0x0A,
				0x0A,
				0x02,
				0x02,
				0x02,
				// GENERATOR-END: MemorySizes
			};

			for (int i = 0; i < memSizes.Length; i++) {
				var keywords = data[i] switch {
					// GENERATOR-BEGIN: Switch
					// ⚠️This was generated by GENERATOR!🦹‍♂️
					0 => "",
					1 => "byte ptr ",
					2 => "dword bcst ",
					3 => "dword ptr ",
					4 => "fpuenv14 ptr ",
					5 => "fpuenv28 ptr ",
					6 => "fpustate108 ptr ",
					7 => "fpustate94 ptr ",
					8 => "fword ptr ",
					9 => "oword ptr ",
					10 => "qword bcst ",
					11 => "qword ptr ",
					12 => "tbyte ptr ",
					13 => "word ptr ",
					14 => "xmmword ptr ",
					15 => "ymmword ptr ",
					16 => "zmmword ptr ",
					17 => "mem384 ptr ",
					// GENERATOR-END: Switch
					_ => throw new InvalidOperationException(),
				};
				memSizes[i] = keywords;
			}

			return memSizes;
		}
	}
}
#endif
