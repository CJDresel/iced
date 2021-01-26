// SPDX-License-Identifier: MIT
// Copyright wtfsckgh@gmail.com
// Copyright iced contributors

#if INSTR_INFO
using System;
using System.Collections.Generic;
using Iced.Intel;

namespace Iced.UnitTests.Intel {
	static partial class ToEnumConverter {
		public static bool TryFlowControl(string value, out FlowControl flowControl) => flowControlDict.TryGetValue(value, out flowControl);
		public static FlowControl GetFlowControl(string value) => TryFlowControl(value, out var flowControl) ? flowControl : throw new InvalidOperationException($"Invalid FlowControl value: {value}");

		static readonly Dictionary<string, FlowControl> flowControlDict =
			// GENERATOR-BEGIN: FlowControlHash
			// ⚠️This was generated by GENERATOR!🦹‍♂️
			new Dictionary<string, FlowControl>(10, StringComparer.Ordinal) {
				{ "Next", FlowControl.Next },
				{ "UnconditionalBranch", FlowControl.UnconditionalBranch },
				{ "IndirectBranch", FlowControl.IndirectBranch },
				{ "ConditionalBranch", FlowControl.ConditionalBranch },
				{ "Return", FlowControl.Return },
				{ "Call", FlowControl.Call },
				{ "IndirectCall", FlowControl.IndirectCall },
				{ "Interrupt", FlowControl.Interrupt },
				{ "XbeginXabortXend", FlowControl.XbeginXabortXend },
				{ "Exception", FlowControl.Exception },
			};
			// GENERATOR-END: FlowControlHash
	}
}
#endif
