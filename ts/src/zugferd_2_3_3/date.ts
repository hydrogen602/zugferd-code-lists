export enum Date {
  /**
   * CCYYMMDD
   *
   * Calendar date: C = Century ; Y = Year ; M = Month ; D = Day.
   */
  Ccyymmdd = "102",
  /**
   * CCYYMMDDHHMMZHHMM
   *
   * Calendar date including time and time zone expressed in hours and minutes. ZHHMM = time zone given as offset from Coordinated Universal Time (UTC).
   */
  Ccyymmddhhmmzhhmm = "205",
}

export function description(value: Date): string {
  switch (value) {
    case Date.Ccyymmdd:
      return "CCYYMMDD";
    case Date.Ccyymmddhhmmzhhmm:
      return "CCYYMMDDHHMMZHHMM";
  }
}
