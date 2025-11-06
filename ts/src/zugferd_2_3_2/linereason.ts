export enum LineReason {
  /**
   * Regular item position (standard case)
   */
  RegularItemPositionStandardCase = "DETAIL",
  /**
   * Subtotal or group
   */
  SubtotalOrGroup = "GROUP",
  /**
   * For information only
   */
  ForInformationOnly = "INFORMATION",
}

export function description(value: LineReason): string {
  switch (value) {
    case LineReason.RegularItemPositionStandardCase:
      return "Regular item position (standard case)";
    case LineReason.SubtotalOrGroup:
      return "Subtotal or group";
    case LineReason.ForInformationOnly:
      return "For information only";
  }
}
