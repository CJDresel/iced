// SPDX-License-Identifier: MIT
// Copyright wtfsckgh@gmail.com
// Copyright iced contributors

using System;
using System.Linq;

namespace Generator.Constants.Encoder {
	[TypeGen(TypeGenOrders.NoDeps)]
	sealed class OpCodeInfoKeysType {
		OpCodeInfoKeysType(GenTypes genTypes) {
			var type = new ConstantsType(TypeIds.OpCodeInfoKeys, ConstantsTypeFlags.None, null, GetConstants());
			genTypes.Add(type);
		}

		static Constant[] GetConstants() =>
			typeof(OpCodeInfoKeywordKeys).GetFields().Where(a => a.IsLiteral).OrderBy(a => a.MetadataToken).
				Select(a => new Constant(ConstantKind.String, a.Name, a.GetRawConstantValue() ?? throw new InvalidOperationException())).ToArray();
	}
}
