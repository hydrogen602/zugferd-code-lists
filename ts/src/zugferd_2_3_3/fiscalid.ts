export enum FiscalID {
  /**
   * Fiscal number
   */
  FiscalNumber = "FC",
}

export function description(value: FiscalID): string {
  switch (value) {
    case FiscalID.FiscalNumber:
      return "Fiscal number";
  }
}
