export enum HybridConformance {
  /**
   * The included document uses a MINIMUM profile
   *
   * Only applicable in Factur-X/ZUGFeRD
   *
   * Not allowed in Germany from 2025-01-01
   */
  TheIncludedDocumentUsesAMinimumProfile = "MINIMUM",
  /**
   * The included document uses a Basic Without Lines profile
   *
   * Only applicable in Factur-X/ZUGFeRD
   *
   * Not allowed in Germany from 2025-01-01
   */
  TheIncludedDocumentUsesABasicWithoutLinesProfile = "BASIC WL",
  /**
   * The included document uses a Basic profile
   *
   * Applicable in Factur-X/ZUGFeRD and Order-X. For Factur-X/ZUGFeRD the BASIC profile is compliant to the EN16931.
   */
  TheIncludedDocumentUsesABasicProfile = "BASIC",
  /**
   * The included document uses a Comfort profile
   *
   * Only applicable in Order-X
   */
  TheIncludedDocumentUsesAComfortProfile = "COMFORT",
  /**
   * The included document uses a EN 16931 profile
   *
   * Only applicable in Factur-X/ZUGFeRD. This profile is compliant to the EN16931.
   */
  TheIncludedDocumentUsesAEn16931Profile = "EN 16931",
  /**
   * The included document uses a Comfort profile
   *
   * Applicable in Factur-X/ZUGFeRD and Order-X. For Factur-X/ZUGFeRD the EXTENDED profile is compliant to and conformant extension of the EN16931.
   */
  TheIncludedDocumentUsesAComfortProfile_Dup = "EXTENDED",
  /**
   * The included document uses an XRECHNUNG profile
   *
   * Only applicable in Factur-X/ZUGFeRD.
   *
   * Not applicable in France
   */
  TheIncludedDocumentUsesAnXrechnungProfile = "XRECHNUNG",
}

export function description(value: HybridConformance): string {
  switch (value) {
    case HybridConformance.TheIncludedDocumentUsesAMinimumProfile:
      return "The included document uses a MINIMUM profile";
    case HybridConformance.TheIncludedDocumentUsesABasicWithoutLinesProfile:
      return "The included document uses a Basic Without Lines profile";
    case HybridConformance.TheIncludedDocumentUsesABasicProfile:
      return "The included document uses a Basic profile";
    case HybridConformance.TheIncludedDocumentUsesAComfortProfile:
      return "The included document uses a Comfort profile";
    case HybridConformance.TheIncludedDocumentUsesAEn16931Profile:
      return "The included document uses a EN 16931 profile";
    case HybridConformance.TheIncludedDocumentUsesAComfortProfile_Dup:
      return "The included document uses a Comfort profile";
    case HybridConformance.TheIncludedDocumentUsesAnXrechnungProfile:
      return "The included document uses an XRECHNUNG profile";
  }
}
