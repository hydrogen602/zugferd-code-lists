export enum Enum1001 {
  /**
   * Request for payment
   *
   * Invoice
   */
  RequestForPayment = "71",
  /**
   * Debit note related to goods or services
   *
   * Invoice
   */
  DebitNoteRelatedToGoodsOrServices = "80",
  /**
   * Credit note related to goods or services
   *
   * Credit Note
   */
  CreditNoteRelatedToGoodsOrServices = "81",
  /**
   * Metered services invoice
   *
   * Invoice
   */
  MeteredServicesInvoice = "82",
  /**
   * Credit note related to financial adjustments
   *
   * Credit Note
   */
  CreditNoteRelatedToFinancialAdjustments = "83",
  /**
   * Debit note related to financial adjustments
   *
   * Invoice
   */
  DebitNoteRelatedToFinancialAdjustments = "84",
  /**
   * Tax notification
   *
   * Invoice
   */
  TaxNotification = "102",
  /**
   * Invoicing data sheet
   *
   * Invoice
   */
  InvoicingDataSheet = "130",
  /**
   * Direct payment valuation
   *
   * Invoice
   */
  DirectPaymentValuation = "202",
  /**
   * Provisional payment valuation
   *
   * Invoice
   */
  ProvisionalPaymentValuation = "203",
  /**
   * Payment valuation
   *
   * Invoice
   */
  PaymentValuation = "204",
  /**
   * Interim application for payment
   *
   * Invoice
   */
  InterimApplicationForPayment = "211",
  /**
   * Final payment request based on completion of work
   *
   * Invoice
   */
  FinalPaymentRequestBasedOnCompletionWork = "218",
  /**
   * Payment request for completed units
   *
   * Invoice
   */
  PaymentRequestForCompletedUnits = "219",
  /**
   * Self billed credit note
   *
   * Credit Note
   */
  SelfBilledCreditNote = "261",
  /**
   * Consolidated credit note - goods and services
   *
   * Credit Note
   */
  ConsolidatedCreditNoteGoodsAndServices = "262",
  /**
   * Price variation invoice
   *
   * Invoice
   */
  PriceVariationInvoice = "295",
  /**
   * Credit note for price variation
   *
   * Credit Note
   */
  CreditNoteForPriceVariation = "296",
  /**
   * Delcredere credit note
   *
   * Credit Note
   */
  DelcredereCreditNote = "308",
  /**
   * Proforma invoice
   *
   * Invoice
   */
  ProformaInvoice = "325",
  /**
   * Partial invoice
   *
   * Invoice
   */
  PartialInvoice = "326",
  /**
   * Commercial invoice which includes a packing list
   *
   * Invoice
   */
  CommercialInvoiceWhichIncludesAPackingList = "331",
  /**
   * Commercial invoice
   *
   * Invoice
   */
  CommercialInvoice = "380",
  /**
   * Credit note
   *
   * Credit Note
   */
  CreditNote = "381",
  /**
   * Commission note
   *
   * Invoice
   */
  CommissionNote = "382",
  /**
   * Debit note
   *
   * Invoice
   */
  DebitNote = "383",
  /**
   * Corrected invoice
   *
   * Invoice
   */
  CorrectedInvoice = "384",
  /**
   * Consolidated invoice
   *
   * Invoice
   */
  ConsolidatedInvoice = "385",
  /**
   * Prepayment invoice
   *
   * Invoice
   */
  PrepaymentInvoice = "386",
  /**
   * Hire invoice
   *
   * Invoice
   */
  HireInvoice = "387",
  /**
   * Tax invoice
   *
   * Invoice
   */
  TaxInvoice = "388",
  /**
   * Self-billed invoice
   *
   * Invoice
   */
  SelfBilledInvoice = "389",
  /**
   * Delcredere invoice
   *
   * Invoice
   */
  DelcredereInvoice = "390",
  /**
   * Factored invoice
   *
   * Invoice
   */
  FactoredInvoice = "393",
  /**
   * Lease invoice
   *
   * Invoice
   */
  LeaseInvoice = "394",
  /**
   * Consignment invoice
   *
   * Invoice
   */
  ConsignmentInvoice = "395",
  /**
   * Factored credit note
   *
   * Credit Note
   */
  FactoredCreditNote = "396",
  /**
   * Optical Character Reading (OCR) payment credit note
   *
   * Credit Note
   */
  OpticalCharacterReadingOcrPaymentCreditNote = "420",
  /**
   * Debit advice
   *
   * Invoice
   */
  DebitAdvice = "456",
  /**
   * Reversal of debit
   *
   * Invoice
   */
  ReversalDebit = "457",
  /**
   * Reversal of credit
   *
   * Credit Note
   */
  ReversalCredit = "458",
  /**
   *  Self-billed corrective invoice, invoice type, Corrected
   *
   * Invoice
   */
  SelfBilledCorrectiveInvoiceInvoiceTypeCorrected = "471",
  /**
   *  Factored Corrective Invoice, invoice type, Corrected
   *
   * Invoice
   */
  FactoredCorrectiveInvoiceInvoiceTypeCorrected = "472",
  /**
   *  Self billed Factored corrective invoice, invoice type, Corrected
   *
   * Invoice
   */
  SelfBilledFactoredCorrectiveInvoiceInvoiceTypeCorrected = "473",
  /**
   *  Self Prepayment invoice, invoice type, Original
   *
   * Invoice
   */
  SelfPrepaymentInvoiceInvoiceTypeOriginal = "500",
  /**
   *  Self billed factored invoice, invoice type, Original
   *
   * Invoice
   */
  SelfBilledFactoredInvoiceInvoiceTypeOriginal = "501",
  /**
   *  Self billet factored Credit Note, Credit note type, Corrected
   *
   * Credit Note
   */
  SelfBilletFactoredCreditNoteCreditNoteTypeCorrected = "502",
  /**
   *  Prepayment credit note, credit note type, Corrected
   *
   * Credit Note
   */
  PrepaymentCreditNoteCreditNoteTypeCorrected = "503",
  /**
   * Self billed debit note
   *
   * Invoice
   */
  SelfBilledDebitNote = "527",
  /**
   * Forwarder's credit note
   *
   * Credit Note
   */
  ForwardersCreditNote = "532",
  /**
   * Forwarder's invoice discrepancy report
   *
   * Invoice
   */
  ForwardersInvoiceDiscrepancyReport = "553",
  /**
   * Insurer's invoice
   *
   * Invoice
   */
  InsurersInvoice = "575",
  /**
   * Forwarder's invoice
   *
   * Invoice
   */
  ForwardersInvoice = "623",
  /**
   * Port charges documents
   *
   * Invoice
   */
  PortChargesDocuments = "633",
  /**
   * Invoice information for accounting purposes
   *
   * Invoice
   */
  InvoiceInformationForAccountingPurposes = "751",
  /**
   * Freight invoice
   *
   * Invoice
   */
  FreightInvoice = "780",
  /**
   * Claim notification
   *
   * Invoice
   */
  ClaimNotification = "817",
  /**
   * Consular invoice
   *
   * Invoice
   */
  ConsularInvoice = "870",
  /**
   * Partial construction invoice
   *
   * Invoice
   */
  PartialConstructionInvoice = "875",
  /**
   * Partial final construction invoice
   *
   * Invoice
   */
  PartialFinalConstructionInvoice = "876",
  /**
   * Final construction invoice
   *
   * Invoice
   */
  FinalConstructionInvoice = "877",
  /**
   * Customs invoice
   *
   * Invoice
   */
  CustomsInvoice = "935",
}

export function description(value: Enum1001): string {
  switch (value) {
    case Enum1001.RequestForPayment:
      return "Request for payment";
    case Enum1001.DebitNoteRelatedToGoodsOrServices:
      return "Debit note related to goods or services";
    case Enum1001.CreditNoteRelatedToGoodsOrServices:
      return "Credit note related to goods or services";
    case Enum1001.MeteredServicesInvoice:
      return "Metered services invoice";
    case Enum1001.CreditNoteRelatedToFinancialAdjustments:
      return "Credit note related to financial adjustments";
    case Enum1001.DebitNoteRelatedToFinancialAdjustments:
      return "Debit note related to financial adjustments";
    case Enum1001.TaxNotification:
      return "Tax notification";
    case Enum1001.InvoicingDataSheet:
      return "Invoicing data sheet";
    case Enum1001.DirectPaymentValuation:
      return "Direct payment valuation";
    case Enum1001.ProvisionalPaymentValuation:
      return "Provisional payment valuation";
    case Enum1001.PaymentValuation:
      return "Payment valuation";
    case Enum1001.InterimApplicationForPayment:
      return "Interim application for payment";
    case Enum1001.FinalPaymentRequestBasedOnCompletionWork:
      return "Final payment request based on completion of work";
    case Enum1001.PaymentRequestForCompletedUnits:
      return "Payment request for completed units";
    case Enum1001.SelfBilledCreditNote:
      return "Self billed credit note";
    case Enum1001.ConsolidatedCreditNoteGoodsAndServices:
      return "Consolidated credit note - goods and services";
    case Enum1001.PriceVariationInvoice:
      return "Price variation invoice";
    case Enum1001.CreditNoteForPriceVariation:
      return "Credit note for price variation";
    case Enum1001.DelcredereCreditNote:
      return "Delcredere credit note";
    case Enum1001.ProformaInvoice:
      return "Proforma invoice";
    case Enum1001.PartialInvoice:
      return "Partial invoice";
    case Enum1001.CommercialInvoiceWhichIncludesAPackingList:
      return "Commercial invoice which includes a packing list";
    case Enum1001.CommercialInvoice:
      return "Commercial invoice";
    case Enum1001.CreditNote:
      return "Credit note";
    case Enum1001.CommissionNote:
      return "Commission note";
    case Enum1001.DebitNote:
      return "Debit note";
    case Enum1001.CorrectedInvoice:
      return "Corrected invoice";
    case Enum1001.ConsolidatedInvoice:
      return "Consolidated invoice";
    case Enum1001.PrepaymentInvoice:
      return "Prepayment invoice";
    case Enum1001.HireInvoice:
      return "Hire invoice";
    case Enum1001.TaxInvoice:
      return "Tax invoice";
    case Enum1001.SelfBilledInvoice:
      return "Self-billed invoice";
    case Enum1001.DelcredereInvoice:
      return "Delcredere invoice";
    case Enum1001.FactoredInvoice:
      return "Factored invoice";
    case Enum1001.LeaseInvoice:
      return "Lease invoice";
    case Enum1001.ConsignmentInvoice:
      return "Consignment invoice";
    case Enum1001.FactoredCreditNote:
      return "Factored credit note";
    case Enum1001.OpticalCharacterReadingOcrPaymentCreditNote:
      return "Optical Character Reading (OCR) payment credit note";
    case Enum1001.DebitAdvice:
      return "Debit advice";
    case Enum1001.ReversalDebit:
      return "Reversal of debit";
    case Enum1001.ReversalCredit:
      return "Reversal of credit";
    case Enum1001.SelfBilledCorrectiveInvoiceInvoiceTypeCorrected:
      return " Self-billed corrective invoice, invoice type, Corrected";
    case Enum1001.FactoredCorrectiveInvoiceInvoiceTypeCorrected:
      return " Factored Corrective Invoice, invoice type, Corrected";
    case Enum1001.SelfBilledFactoredCorrectiveInvoiceInvoiceTypeCorrected:
      return " Self billed Factored corrective invoice, invoice type, Corrected";
    case Enum1001.SelfPrepaymentInvoiceInvoiceTypeOriginal:
      return " Self Prepayment invoice, invoice type, Original";
    case Enum1001.SelfBilledFactoredInvoiceInvoiceTypeOriginal:
      return " Self billed factored invoice, invoice type, Original";
    case Enum1001.SelfBilletFactoredCreditNoteCreditNoteTypeCorrected:
      return " Self billet factored Credit Note, Credit note type, Corrected";
    case Enum1001.PrepaymentCreditNoteCreditNoteTypeCorrected:
      return " Prepayment credit note, credit note type, Corrected";
    case Enum1001.SelfBilledDebitNote:
      return "Self billed debit note";
    case Enum1001.ForwardersCreditNote:
      return "Forwarder's credit note";
    case Enum1001.ForwardersInvoiceDiscrepancyReport:
      return "Forwarder's invoice discrepancy report";
    case Enum1001.InsurersInvoice:
      return "Insurer's invoice";
    case Enum1001.ForwardersInvoice:
      return "Forwarder's invoice";
    case Enum1001.PortChargesDocuments:
      return "Port charges documents";
    case Enum1001.InvoiceInformationForAccountingPurposes:
      return "Invoice information for accounting purposes";
    case Enum1001.FreightInvoice:
      return "Freight invoice";
    case Enum1001.ClaimNotification:
      return "Claim notification";
    case Enum1001.ConsularInvoice:
      return "Consular invoice";
    case Enum1001.PartialConstructionInvoice:
      return "Partial construction invoice";
    case Enum1001.PartialFinalConstructionInvoice:
      return "Partial final construction invoice";
    case Enum1001.FinalConstructionInvoice:
      return "Final construction invoice";
    case Enum1001.CustomsInvoice:
      return "Customs invoice";
  }
}
