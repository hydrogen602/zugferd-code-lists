#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Enum1001 {
    /// Request for payment
    RequestForPayment,
    /// Debit note related to goods or services
    DebitNoteRelatedToGoodsOrServices,
    /// Credit note related to goods or services
    CreditNoteRelatedToGoodsOrServices,
    /// Metered services invoice
    MeteredServicesInvoice,
    /// Credit note related to financial adjustments
    CreditNoteRelatedToFinancialAdjustments,
    /// Debit note related to financial adjustments
    DebitNoteRelatedToFinancialAdjustments,
    /// Tax notification
    TaxNotification,
    /// Invoicing data sheet
    InvoicingDataSheet,
    /// Direct payment valuation
    DirectPaymentValuation,
    /// Provisional payment valuation
    ProvisionalPaymentValuation,
    /// Payment valuation
    PaymentValuation,
    /// Interim application for payment
    InterimApplicationForPayment,
    /// Final payment request based on completion of work
    FinalPaymentRequestBasedOnCompletionWork,
    /// Payment request for completed units
    PaymentRequestForCompletedUnits,
    /// Self billed credit note
    SelfBilledCreditNote,
    /// Consolidated credit note - goods and services
    ConsolidatedCreditNoteGoodsAndServices,
    /// Price variation invoice
    PriceVariationInvoice,
    /// Credit note for price variation
    CreditNoteForPriceVariation,
    /// Delcredere credit note
    DelcredereCreditNote,
    /// Proforma invoice
    ProformaInvoice,
    /// Partial invoice
    PartialInvoice,
    /// Commercial invoice which includes a packing list
    CommercialInvoiceWhichIncludesAPackingList,
    /// Commercial invoice
    CommercialInvoice,
    /// Credit note
    CreditNote,
    /// Commission note
    CommissionNote,
    /// Debit note
    DebitNote,
    /// Corrected invoice
    CorrectedInvoice,
    /// Consolidated invoice
    ConsolidatedInvoice,
    /// Prepayment invoice
    PrepaymentInvoice,
    /// Hire invoice
    HireInvoice,
    /// Tax invoice
    TaxInvoice,
    /// Self-billed invoice
    SelfBilledInvoice,
    /// Delcredere invoice
    DelcredereInvoice,
    /// Factored invoice
    FactoredInvoice,
    /// Lease invoice
    LeaseInvoice,
    /// Consignment invoice
    ConsignmentInvoice,
    /// Factored credit note
    FactoredCreditNote,
    /// Optical Character Reading (OCR) payment credit note
    OpticalCharacterReadingOcrPaymentCreditNote,
    /// Debit advice
    DebitAdvice,
    /// Reversal of debit
    ReversalDebit,
    /// Reversal of credit
    ReversalCredit,
    /// Self billed debit note
    SelfBilledDebitNote,
    /// Forwarder's credit note
    ForwardersCreditNote,
    /// Forwarder's invoice discrepancy report
    ForwardersInvoiceDiscrepancyReport,
    /// Insurer's invoice
    InsurersInvoice,
    /// Forwarder's invoice
    ForwardersInvoice,
    /// Port charges documents
    PortChargesDocuments,
    /// Invoice information for accounting purposes
    InvoiceInformationForAccountingPurposes,
    /// Freight invoice
    FreightInvoice,
    /// Claim notification
    ClaimNotification,
    /// Consular invoice
    ConsularInvoice,
    /// Partial construction invoice
    PartialConstructionInvoice,
    /// Partial final construction invoice
    PartialFinalConstructionInvoice,
    /// Final construction invoice
    FinalConstructionInvoice,
    /// Customs invoice
    CustomsInvoice,
}

impl crate::Code for Enum1001 {
    fn code(&self) -> &str {
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
    fn description(&self) -> &str {
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
