export enum TimeCII {
  /**
   * Date of invoice
   */
  DateInvoice = "5",
  /**
   * Date of delivery of goods to establishments/domicile/site
   */
  DateDeliveryGoodsToEstablishmentsDomicileSite = "29",
  /**
   * Payment date
   */
  PaymentDate = "72",
}

export function description(value: TimeCII): string {
  switch (value) {
    case TimeCII.DateInvoice:
      return "Date of invoice";
    case TimeCII.DateDeliveryGoodsToEstablishmentsDomicileSite:
      return "Date of delivery of goods to establishments/domicile/site";
    case TimeCII.PaymentDate:
      return "Payment date";
  }
}
