// SPDX-License-Identifier: MIT
// Copyright wtfsckgh@gmail.com
// Copyright iced contributors

// ⚠️This file was generated by GENERATOR!🦹‍♂️

#nullable enable

#if DECODER
using System;

namespace Iced.Intel.DecoderInternal {
	[Flags]
	enum HandlerFlags : uint {
		None = 0x00000000,
		Xacquire = 0x00000001,
		Xrelease = 0x00000002,
		XacquireXreleaseNoLock = 0x00000004,
		Lock = 0x00000008,
	}
}
#endif
