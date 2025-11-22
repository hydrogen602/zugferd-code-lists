#![allow(non_camel_case_types)]

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Enum1001 {
    /// Request for payment
    ///
    /// Invoice
    RequestForPayment,
    /// Debit note related to goods or services
    ///
    /// Invoice
    DebitNoteRelatedToGoodsOrServices,
    /// Credit note related to goods or services
    ///
    /// Credit Note
    CreditNoteRelatedToGoodsOrServices,
    /// Metered services invoice
    ///
    /// Invoice
    MeteredServicesInvoice,
    /// Credit note related to financial adjustments
    ///
    /// Credit Note
    CreditNoteRelatedToFinancialAdjustments,
    /// Debit note related to financial adjustments
    ///
    /// Invoice
    DebitNoteRelatedToFinancialAdjustments,
    /// Tax notification
    ///
    /// Invoice
    TaxNotification,
    /// Invoicing data sheet
    ///
    /// Invoice
    InvoicingDataSheet,
    /// Direct payment valuation
    ///
    /// Invoice
    DirectPaymentValuation,
    /// Provisional payment valuation
    ///
    /// Invoice
    ProvisionalPaymentValuation,
    /// Payment valuation
    ///
    /// Invoice
    PaymentValuation,
    /// Interim application for payment
    ///
    /// Invoice
    InterimApplicationForPayment,
    /// Final payment request based on completion of work
    ///
    /// Invoice
    FinalPaymentRequestBasedOnCompletionWork,
    /// Payment request for completed units
    ///
    /// Invoice
    PaymentRequestForCompletedUnits,
    /// Self billed credit note
    ///
    /// Credit Note
    SelfBilledCreditNote,
    /// Consolidated credit note - goods and services
    ///
    /// Credit Note
    ConsolidatedCreditNoteGoodsAndServices,
    /// Price variation invoice
    ///
    /// Invoice
    PriceVariationInvoice,
    /// Credit note for price variation
    ///
    /// Credit Note
    CreditNoteForPriceVariation,
    /// Delcredere credit note
    ///
    /// Credit Note
    DelcredereCreditNote,
    /// Proforma invoice
    ///
    /// Invoice
    ProformaInvoice,
    /// Partial invoice
    ///
    /// Invoice
    PartialInvoice,
    /// Commercial invoice which includes a packing list
    ///
    /// Invoice
    CommercialInvoiceWhichIncludesAPackingList,
    /// Commercial invoice
    ///
    /// Invoice
    CommercialInvoice,
    /// Credit note
    ///
    /// Credit Note
    CreditNote,
    /// Commission note
    ///
    /// Invoice
    CommissionNote,
    /// Debit note
    ///
    /// Invoice
    DebitNote,
    /// Corrected invoice
    ///
    /// Invoice
    CorrectedInvoice,
    /// Consolidated invoice
    ///
    /// Invoice
    ConsolidatedInvoice,
    /// Prepayment invoice
    ///
    /// Invoice
    PrepaymentInvoice,
    /// Hire invoice
    ///
    /// Invoice
    HireInvoice,
    /// Tax invoice
    ///
    /// Invoice
    TaxInvoice,
    /// Self-billed invoice
    ///
    /// Invoice
    SelfBilledInvoice,
    /// Delcredere invoice
    ///
    /// Invoice
    DelcredereInvoice,
    /// Factored invoice
    ///
    /// Invoice
    FactoredInvoice,
    /// Lease invoice
    ///
    /// Invoice
    LeaseInvoice,
    /// Consignment invoice
    ///
    /// Invoice
    ConsignmentInvoice,
    /// Factored credit note
    ///
    /// Credit Note
    FactoredCreditNote,
    /// Optical Character Reading (OCR) payment credit note
    ///
    /// Credit Note
    OpticalCharacterReadingOcrPaymentCreditNote,
    /// Debit advice
    ///
    /// Invoice
    DebitAdvice,
    /// Reversal of debit
    ///
    /// Invoice
    ReversalDebit,
    /// Reversal of credit
    ///
    /// Credit Note
    ReversalCredit,
    ///  Self-billed corrective invoice, invoice type, Corrected
    ///
    /// Invoice
    SelfBilledCorrectiveInvoiceInvoiceTypeCorrected,
    ///  Factored Corrective Invoice, invoice type, Corrected
    ///
    /// Invoice
    FactoredCorrectiveInvoiceInvoiceTypeCorrected,
    ///  Self billed Factored corrective invoice, invoice type, Corrected
    ///
    /// Invoice
    SelfBilledFactoredCorrectiveInvoiceInvoiceTypeCorrected,
    ///  Self Prepayment invoice, invoice type, Original
    ///
    /// Invoice
    SelfPrepaymentInvoiceInvoiceTypeOriginal,
    ///  Self billed factored invoice, invoice type, Original
    ///
    /// Invoice
    SelfBilledFactoredInvoiceInvoiceTypeOriginal,
    ///  Self billet factored Credit Note, Credit note type, Corrected
    ///
    /// Credit Note
    SelfBilletFactoredCreditNoteCreditNoteTypeCorrected,
    ///  Prepayment credit note, credit note type, Corrected
    ///
    /// Credit Note
    PrepaymentCreditNoteCreditNoteTypeCorrected,
    /// Self billed debit note
    ///
    /// Invoice
    SelfBilledDebitNote,
    /// Forwarder's credit note
    ///
    /// Credit Note
    ForwardersCreditNote,
    /// Forwarder's invoice discrepancy report
    ///
    /// Invoice
    ForwardersInvoiceDiscrepancyReport,
    /// Insurer's invoice
    ///
    /// Invoice
    InsurersInvoice,
    /// Forwarder's invoice
    ///
    /// Invoice
    ForwardersInvoice,
    /// Port charges documents
    ///
    /// Invoice
    PortChargesDocuments,
    /// Invoice information for accounting purposes
    ///
    /// Invoice
    InvoiceInformationForAccountingPurposes,
    /// Freight invoice
    ///
    /// Invoice
    FreightInvoice,
    /// Claim notification
    ///
    /// Invoice
    ClaimNotification,
    /// Consular invoice
    ///
    /// Invoice
    ConsularInvoice,
    /// Partial construction invoice
    ///
    /// Invoice
    PartialConstructionInvoice,
    /// Partial final construction invoice
    ///
    /// Invoice
    PartialFinalConstructionInvoice,
    /// Final construction invoice
    ///
    /// Invoice
    FinalConstructionInvoice,
    /// Customs invoice
    ///
    /// Invoice
    CustomsInvoice,
}

impl std::fmt::Display for Enum1001 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as crate::Code>::code(*self))
    }
}

impl std::str::FromStr for Enum1001 {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as crate::FromCode>::from_code(s).ok_or(())
    }
}

impl crate::Code for Enum1001 {
    fn code(self) -> &'static str {
        match self {
            Enum1001::RequestForPayment => "71",
            Enum1001::DebitNoteRelatedToGoodsOrServices => "80",
            Enum1001::CreditNoteRelatedToGoodsOrServices => "81",
            Enum1001::MeteredServicesInvoice => "82",
            Enum1001::CreditNoteRelatedToFinancialAdjustments => "83",
            Enum1001::DebitNoteRelatedToFinancialAdjustments => "84",
            Enum1001::TaxNotification => "102",
            Enum1001::InvoicingDataSheet => "130",
            Enum1001::DirectPaymentValuation => "202",
            Enum1001::ProvisionalPaymentValuation => "203",
            Enum1001::PaymentValuation => "204",
            Enum1001::InterimApplicationForPayment => "211",
            Enum1001::FinalPaymentRequestBasedOnCompletionWork => "218",
            Enum1001::PaymentRequestForCompletedUnits => "219",
            Enum1001::SelfBilledCreditNote => "261",
            Enum1001::ConsolidatedCreditNoteGoodsAndServices => "262",
            Enum1001::PriceVariationInvoice => "295",
            Enum1001::CreditNoteForPriceVariation => "296",
            Enum1001::DelcredereCreditNote => "308",
            Enum1001::ProformaInvoice => "325",
            Enum1001::PartialInvoice => "326",
            Enum1001::CommercialInvoiceWhichIncludesAPackingList => "331",
            Enum1001::CommercialInvoice => "380",
            Enum1001::CreditNote => "381",
            Enum1001::CommissionNote => "382",
            Enum1001::DebitNote => "383",
            Enum1001::CorrectedInvoice => "384",
            Enum1001::ConsolidatedInvoice => "385",
            Enum1001::PrepaymentInvoice => "386",
            Enum1001::HireInvoice => "387",
            Enum1001::TaxInvoice => "388",
            Enum1001::SelfBilledInvoice => "389",
            Enum1001::DelcredereInvoice => "390",
            Enum1001::FactoredInvoice => "393",
            Enum1001::LeaseInvoice => "394",
            Enum1001::ConsignmentInvoice => "395",
            Enum1001::FactoredCreditNote => "396",
            Enum1001::OpticalCharacterReadingOcrPaymentCreditNote => "420",
            Enum1001::DebitAdvice => "456",
            Enum1001::ReversalDebit => "457",
            Enum1001::ReversalCredit => "458",
            Enum1001::SelfBilledCorrectiveInvoiceInvoiceTypeCorrected => "471",
            Enum1001::FactoredCorrectiveInvoiceInvoiceTypeCorrected => "472",
            Enum1001::SelfBilledFactoredCorrectiveInvoiceInvoiceTypeCorrected => "473",
            Enum1001::SelfPrepaymentInvoiceInvoiceTypeOriginal => "500",
            Enum1001::SelfBilledFactoredInvoiceInvoiceTypeOriginal => "501",
            Enum1001::SelfBilletFactoredCreditNoteCreditNoteTypeCorrected => "502",
            Enum1001::PrepaymentCreditNoteCreditNoteTypeCorrected => "503",
            Enum1001::SelfBilledDebitNote => "527",
            Enum1001::ForwardersCreditNote => "532",
            Enum1001::ForwardersInvoiceDiscrepancyReport => "553",
            Enum1001::InsurersInvoice => "575",
            Enum1001::ForwardersInvoice => "623",
            Enum1001::PortChargesDocuments => "633",
            Enum1001::InvoiceInformationForAccountingPurposes => "751",
            Enum1001::FreightInvoice => "780",
            Enum1001::ClaimNotification => "817",
            Enum1001::ConsularInvoice => "870",
            Enum1001::PartialConstructionInvoice => "875",
            Enum1001::PartialFinalConstructionInvoice => "876",
            Enum1001::FinalConstructionInvoice => "877",
            Enum1001::CustomsInvoice => "935",
        }
    }
}

impl crate::Description for Enum1001 {
    fn description(self) -> &'static str {
        match self {
            Enum1001::RequestForPayment => "Request for payment",
            Enum1001::DebitNoteRelatedToGoodsOrServices => {
                "Debit note related to goods or services"
            }
            Enum1001::CreditNoteRelatedToGoodsOrServices => {
                "Credit note related to goods or services"
            }
            Enum1001::MeteredServicesInvoice => "Metered services invoice",
            Enum1001::CreditNoteRelatedToFinancialAdjustments => {
                "Credit note related to financial adjustments"
            }
            Enum1001::DebitNoteRelatedToFinancialAdjustments => {
                "Debit note related to financial adjustments"
            }
            Enum1001::TaxNotification => "Tax notification",
            Enum1001::InvoicingDataSheet => "Invoicing data sheet",
            Enum1001::DirectPaymentValuation => "Direct payment valuation",
            Enum1001::ProvisionalPaymentValuation => "Provisional payment valuation",
            Enum1001::PaymentValuation => "Payment valuation",
            Enum1001::InterimApplicationForPayment => "Interim application for payment",
            Enum1001::FinalPaymentRequestBasedOnCompletionWork => {
                "Final payment request based on completion of work"
            }
            Enum1001::PaymentRequestForCompletedUnits => "Payment request for completed units",
            Enum1001::SelfBilledCreditNote => "Self billed credit note",
            Enum1001::ConsolidatedCreditNoteGoodsAndServices => {
                "Consolidated credit note - goods and services"
            }
            Enum1001::PriceVariationInvoice => "Price variation invoice",
            Enum1001::CreditNoteForPriceVariation => "Credit note for price variation",
            Enum1001::DelcredereCreditNote => "Delcredere credit note",
            Enum1001::ProformaInvoice => "Proforma invoice",
            Enum1001::PartialInvoice => "Partial invoice",
            Enum1001::CommercialInvoiceWhichIncludesAPackingList => {
                "Commercial invoice which includes a packing list"
            }
            Enum1001::CommercialInvoice => "Commercial invoice",
            Enum1001::CreditNote => "Credit note",
            Enum1001::CommissionNote => "Commission note",
            Enum1001::DebitNote => "Debit note",
            Enum1001::CorrectedInvoice => "Corrected invoice",
            Enum1001::ConsolidatedInvoice => "Consolidated invoice",
            Enum1001::PrepaymentInvoice => "Prepayment invoice",
            Enum1001::HireInvoice => "Hire invoice",
            Enum1001::TaxInvoice => "Tax invoice",
            Enum1001::SelfBilledInvoice => "Self-billed invoice",
            Enum1001::DelcredereInvoice => "Delcredere invoice",
            Enum1001::FactoredInvoice => "Factored invoice",
            Enum1001::LeaseInvoice => "Lease invoice",
            Enum1001::ConsignmentInvoice => "Consignment invoice",
            Enum1001::FactoredCreditNote => "Factored credit note",
            Enum1001::OpticalCharacterReadingOcrPaymentCreditNote => {
                "Optical Character Reading (OCR) payment credit note"
            }
            Enum1001::DebitAdvice => "Debit advice",
            Enum1001::ReversalDebit => "Reversal of debit",
            Enum1001::ReversalCredit => "Reversal of credit",
            Enum1001::SelfBilledCorrectiveInvoiceInvoiceTypeCorrected => {
                " Self-billed corrective invoice, invoice type, Corrected"
            }
            Enum1001::FactoredCorrectiveInvoiceInvoiceTypeCorrected => {
                " Factored Corrective Invoice, invoice type, Corrected"
            }
            Enum1001::SelfBilledFactoredCorrectiveInvoiceInvoiceTypeCorrected => {
                " Self billed Factored corrective invoice, invoice type, Corrected"
            }
            Enum1001::SelfPrepaymentInvoiceInvoiceTypeOriginal => {
                " Self Prepayment invoice, invoice type, Original"
            }
            Enum1001::SelfBilledFactoredInvoiceInvoiceTypeOriginal => {
                " Self billed factored invoice, invoice type, Original"
            }
            Enum1001::SelfBilletFactoredCreditNoteCreditNoteTypeCorrected => {
                " Self billet factored Credit Note, Credit note type, Corrected"
            }
            Enum1001::PrepaymentCreditNoteCreditNoteTypeCorrected => {
                " Prepayment credit note, credit note type, Corrected"
            }
            Enum1001::SelfBilledDebitNote => "Self billed debit note",
            Enum1001::ForwardersCreditNote => "Forwarder's credit note",
            Enum1001::ForwardersInvoiceDiscrepancyReport => {
                "Forwarder's invoice discrepancy report"
            }
            Enum1001::InsurersInvoice => "Insurer's invoice",
            Enum1001::ForwardersInvoice => "Forwarder's invoice",
            Enum1001::PortChargesDocuments => "Port charges documents",
            Enum1001::InvoiceInformationForAccountingPurposes => {
                "Invoice information for accounting purposes"
            }
            Enum1001::FreightInvoice => "Freight invoice",
            Enum1001::ClaimNotification => "Claim notification",
            Enum1001::ConsularInvoice => "Consular invoice",
            Enum1001::PartialConstructionInvoice => "Partial construction invoice",
            Enum1001::PartialFinalConstructionInvoice => "Partial final construction invoice",
            Enum1001::FinalConstructionInvoice => "Final construction invoice",
            Enum1001::CustomsInvoice => "Customs invoice",
        }
    }
}

impl crate::FromCode for Enum1001 {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "71" => Some(Enum1001::RequestForPayment),
            "80" => Some(Enum1001::DebitNoteRelatedToGoodsOrServices),
            "81" => Some(Enum1001::CreditNoteRelatedToGoodsOrServices),
            "82" => Some(Enum1001::MeteredServicesInvoice),
            "83" => Some(Enum1001::CreditNoteRelatedToFinancialAdjustments),
            "84" => Some(Enum1001::DebitNoteRelatedToFinancialAdjustments),
            "102" => Some(Enum1001::TaxNotification),
            "130" => Some(Enum1001::InvoicingDataSheet),
            "202" => Some(Enum1001::DirectPaymentValuation),
            "203" => Some(Enum1001::ProvisionalPaymentValuation),
            "204" => Some(Enum1001::PaymentValuation),
            "211" => Some(Enum1001::InterimApplicationForPayment),
            "218" => Some(Enum1001::FinalPaymentRequestBasedOnCompletionWork),
            "219" => Some(Enum1001::PaymentRequestForCompletedUnits),
            "261" => Some(Enum1001::SelfBilledCreditNote),
            "262" => Some(Enum1001::ConsolidatedCreditNoteGoodsAndServices),
            "295" => Some(Enum1001::PriceVariationInvoice),
            "296" => Some(Enum1001::CreditNoteForPriceVariation),
            "308" => Some(Enum1001::DelcredereCreditNote),
            "325" => Some(Enum1001::ProformaInvoice),
            "326" => Some(Enum1001::PartialInvoice),
            "331" => Some(Enum1001::CommercialInvoiceWhichIncludesAPackingList),
            "380" => Some(Enum1001::CommercialInvoice),
            "381" => Some(Enum1001::CreditNote),
            "382" => Some(Enum1001::CommissionNote),
            "383" => Some(Enum1001::DebitNote),
            "384" => Some(Enum1001::CorrectedInvoice),
            "385" => Some(Enum1001::ConsolidatedInvoice),
            "386" => Some(Enum1001::PrepaymentInvoice),
            "387" => Some(Enum1001::HireInvoice),
            "388" => Some(Enum1001::TaxInvoice),
            "389" => Some(Enum1001::SelfBilledInvoice),
            "390" => Some(Enum1001::DelcredereInvoice),
            "393" => Some(Enum1001::FactoredInvoice),
            "394" => Some(Enum1001::LeaseInvoice),
            "395" => Some(Enum1001::ConsignmentInvoice),
            "396" => Some(Enum1001::FactoredCreditNote),
            "420" => Some(Enum1001::OpticalCharacterReadingOcrPaymentCreditNote),
            "456" => Some(Enum1001::DebitAdvice),
            "457" => Some(Enum1001::ReversalDebit),
            "458" => Some(Enum1001::ReversalCredit),
            "471" => Some(Enum1001::SelfBilledCorrectiveInvoiceInvoiceTypeCorrected),
            "472" => Some(Enum1001::FactoredCorrectiveInvoiceInvoiceTypeCorrected),
            "473" => Some(Enum1001::SelfBilledFactoredCorrectiveInvoiceInvoiceTypeCorrected),
            "500" => Some(Enum1001::SelfPrepaymentInvoiceInvoiceTypeOriginal),
            "501" => Some(Enum1001::SelfBilledFactoredInvoiceInvoiceTypeOriginal),
            "502" => Some(Enum1001::SelfBilletFactoredCreditNoteCreditNoteTypeCorrected),
            "503" => Some(Enum1001::PrepaymentCreditNoteCreditNoteTypeCorrected),
            "527" => Some(Enum1001::SelfBilledDebitNote),
            "532" => Some(Enum1001::ForwardersCreditNote),
            "553" => Some(Enum1001::ForwardersInvoiceDiscrepancyReport),
            "575" => Some(Enum1001::InsurersInvoice),
            "623" => Some(Enum1001::ForwardersInvoice),
            "633" => Some(Enum1001::PortChargesDocuments),
            "751" => Some(Enum1001::InvoiceInformationForAccountingPurposes),
            "780" => Some(Enum1001::FreightInvoice),
            "817" => Some(Enum1001::ClaimNotification),
            "870" => Some(Enum1001::ConsularInvoice),
            "875" => Some(Enum1001::PartialConstructionInvoice),
            "876" => Some(Enum1001::PartialFinalConstructionInvoice),
            "877" => Some(Enum1001::FinalConstructionInvoice),
            "935" => Some(Enum1001::CustomsInvoice),
            _ => None,
        }
    }
}
