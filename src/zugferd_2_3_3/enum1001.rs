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
    type Err = crate::ParseError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as crate::FromCode>::from_code(s)
            .ok_or_else(|| crate::ParseError::<Self>::new(s.to_owned()))
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

// Start: (Version) TryFrom Enum1001 to crate::zugferd_2_3_2::Enum1001
impl std::convert::TryFrom<Enum1001> for crate::zugferd_2_3_2::Enum1001 {
    type Error = ErrFromEnum1001ToCrateZugferd232Enum1001;
    fn try_from(value: Enum1001) -> Result<Self, Self::Error> {
        match value {
            Enum1001::RequestForPayment => Ok(crate::zugferd_2_3_2::Enum1001::RequestForPayment),
            Enum1001::DebitNoteRelatedToGoodsOrServices => Ok(crate::zugferd_2_3_2::Enum1001::DebitNoteRelatedToGoodsOrServices),
            Enum1001::CreditNoteRelatedToGoodsOrServices => Ok(crate::zugferd_2_3_2::Enum1001::CreditNoteRelatedToGoodsOrServices),
            Enum1001::MeteredServicesInvoice => Ok(crate::zugferd_2_3_2::Enum1001::MeteredServicesInvoice),
            Enum1001::CreditNoteRelatedToFinancialAdjustments => Ok(crate::zugferd_2_3_2::Enum1001::CreditNoteRelatedToFinancialAdjustments),
            Enum1001::DebitNoteRelatedToFinancialAdjustments => Ok(crate::zugferd_2_3_2::Enum1001::DebitNoteRelatedToFinancialAdjustments),
            Enum1001::TaxNotification => Ok(crate::zugferd_2_3_2::Enum1001::TaxNotification),
            Enum1001::InvoicingDataSheet => Ok(crate::zugferd_2_3_2::Enum1001::InvoicingDataSheet),
            Enum1001::DirectPaymentValuation => Ok(crate::zugferd_2_3_2::Enum1001::DirectPaymentValuation),
            Enum1001::ProvisionalPaymentValuation => Ok(crate::zugferd_2_3_2::Enum1001::ProvisionalPaymentValuation),
            Enum1001::PaymentValuation => Ok(crate::zugferd_2_3_2::Enum1001::PaymentValuation),
            Enum1001::InterimApplicationForPayment => Ok(crate::zugferd_2_3_2::Enum1001::InterimApplicationForPayment),
            Enum1001::FinalPaymentRequestBasedOnCompletionWork => Ok(crate::zugferd_2_3_2::Enum1001::FinalPaymentRequestBasedOnCompletionWork),
            Enum1001::PaymentRequestForCompletedUnits => Ok(crate::zugferd_2_3_2::Enum1001::PaymentRequestForCompletedUnits),
            Enum1001::SelfBilledCreditNote => Ok(crate::zugferd_2_3_2::Enum1001::SelfBilledCreditNote),
            Enum1001::ConsolidatedCreditNoteGoodsAndServices => Ok(crate::zugferd_2_3_2::Enum1001::ConsolidatedCreditNoteGoodsAndServices),
            Enum1001::PriceVariationInvoice => Ok(crate::zugferd_2_3_2::Enum1001::PriceVariationInvoice),
            Enum1001::CreditNoteForPriceVariation => Ok(crate::zugferd_2_3_2::Enum1001::CreditNoteForPriceVariation),
            Enum1001::DelcredereCreditNote => Ok(crate::zugferd_2_3_2::Enum1001::DelcredereCreditNote),
            Enum1001::ProformaInvoice => Ok(crate::zugferd_2_3_2::Enum1001::ProformaInvoice),
            Enum1001::PartialInvoice => Ok(crate::zugferd_2_3_2::Enum1001::PartialInvoice),
            Enum1001::CommercialInvoiceWhichIncludesAPackingList => Ok(crate::zugferd_2_3_2::Enum1001::CommercialInvoiceWhichIncludesAPackingList),
            Enum1001::CommercialInvoice => Ok(crate::zugferd_2_3_2::Enum1001::CommercialInvoice),
            Enum1001::CreditNote => Ok(crate::zugferd_2_3_2::Enum1001::CreditNote),
            Enum1001::CommissionNote => Ok(crate::zugferd_2_3_2::Enum1001::CommissionNote),
            Enum1001::DebitNote => Ok(crate::zugferd_2_3_2::Enum1001::DebitNote),
            Enum1001::CorrectedInvoice => Ok(crate::zugferd_2_3_2::Enum1001::CorrectedInvoice),
            Enum1001::ConsolidatedInvoice => Ok(crate::zugferd_2_3_2::Enum1001::ConsolidatedInvoice),
            Enum1001::PrepaymentInvoice => Ok(crate::zugferd_2_3_2::Enum1001::PrepaymentInvoice),
            Enum1001::HireInvoice => Ok(crate::zugferd_2_3_2::Enum1001::HireInvoice),
            Enum1001::TaxInvoice => Ok(crate::zugferd_2_3_2::Enum1001::TaxInvoice),
            Enum1001::SelfBilledInvoice => Ok(crate::zugferd_2_3_2::Enum1001::SelfBilledInvoice),
            Enum1001::DelcredereInvoice => Ok(crate::zugferd_2_3_2::Enum1001::DelcredereInvoice),
            Enum1001::FactoredInvoice => Ok(crate::zugferd_2_3_2::Enum1001::FactoredInvoice),
            Enum1001::LeaseInvoice => Ok(crate::zugferd_2_3_2::Enum1001::LeaseInvoice),
            Enum1001::ConsignmentInvoice => Ok(crate::zugferd_2_3_2::Enum1001::ConsignmentInvoice),
            Enum1001::FactoredCreditNote => Ok(crate::zugferd_2_3_2::Enum1001::FactoredCreditNote),
            Enum1001::OpticalCharacterReadingOcrPaymentCreditNote => Ok(crate::zugferd_2_3_2::Enum1001::OpticalCharacterReadingOcrPaymentCreditNote),
            Enum1001::DebitAdvice => Ok(crate::zugferd_2_3_2::Enum1001::DebitAdvice),
            Enum1001::ReversalDebit => Ok(crate::zugferd_2_3_2::Enum1001::ReversalDebit),
            Enum1001::ReversalCredit => Ok(crate::zugferd_2_3_2::Enum1001::ReversalCredit),
            Enum1001::SelfBilledDebitNote => Ok(crate::zugferd_2_3_2::Enum1001::SelfBilledDebitNote),
            Enum1001::ForwardersCreditNote => Ok(crate::zugferd_2_3_2::Enum1001::ForwardersCreditNote),
            Enum1001::ForwardersInvoiceDiscrepancyReport => Ok(crate::zugferd_2_3_2::Enum1001::ForwardersInvoiceDiscrepancyReport),
            Enum1001::InsurersInvoice => Ok(crate::zugferd_2_3_2::Enum1001::InsurersInvoice),
            Enum1001::ForwardersInvoice => Ok(crate::zugferd_2_3_2::Enum1001::ForwardersInvoice),
            Enum1001::PortChargesDocuments => Ok(crate::zugferd_2_3_2::Enum1001::PortChargesDocuments),
            Enum1001::InvoiceInformationForAccountingPurposes => Ok(crate::zugferd_2_3_2::Enum1001::InvoiceInformationForAccountingPurposes),
            Enum1001::FreightInvoice => Ok(crate::zugferd_2_3_2::Enum1001::FreightInvoice),
            Enum1001::ClaimNotification => Ok(crate::zugferd_2_3_2::Enum1001::ClaimNotification),
            Enum1001::ConsularInvoice => Ok(crate::zugferd_2_3_2::Enum1001::ConsularInvoice),
            Enum1001::PartialConstructionInvoice => Ok(crate::zugferd_2_3_2::Enum1001::PartialConstructionInvoice),
            Enum1001::PartialFinalConstructionInvoice => Ok(crate::zugferd_2_3_2::Enum1001::PartialFinalConstructionInvoice),
            Enum1001::FinalConstructionInvoice => Ok(crate::zugferd_2_3_2::Enum1001::FinalConstructionInvoice),
            Enum1001::CustomsInvoice => Ok(crate::zugferd_2_3_2::Enum1001::CustomsInvoice),
            Enum1001::SelfBilledCorrectiveInvoiceInvoiceTypeCorrected => Err(ErrFromEnum1001ToCrateZugferd232Enum1001::SelfBilledCorrectiveInvoiceInvoiceTypeCorrected),
            Enum1001::FactoredCorrectiveInvoiceInvoiceTypeCorrected => Err(ErrFromEnum1001ToCrateZugferd232Enum1001::FactoredCorrectiveInvoiceInvoiceTypeCorrected),
            Enum1001::SelfBilledFactoredCorrectiveInvoiceInvoiceTypeCorrected => Err(ErrFromEnum1001ToCrateZugferd232Enum1001::SelfBilledFactoredCorrectiveInvoiceInvoiceTypeCorrected),
            Enum1001::SelfPrepaymentInvoiceInvoiceTypeOriginal => Err(ErrFromEnum1001ToCrateZugferd232Enum1001::SelfPrepaymentInvoiceInvoiceTypeOriginal),
            Enum1001::SelfBilledFactoredInvoiceInvoiceTypeOriginal => Err(ErrFromEnum1001ToCrateZugferd232Enum1001::SelfBilledFactoredInvoiceInvoiceTypeOriginal),
            Enum1001::SelfBilletFactoredCreditNoteCreditNoteTypeCorrected => Err(ErrFromEnum1001ToCrateZugferd232Enum1001::SelfBilletFactoredCreditNoteCreditNoteTypeCorrected),
            Enum1001::PrepaymentCreditNoteCreditNoteTypeCorrected => Err(ErrFromEnum1001ToCrateZugferd232Enum1001::PrepaymentCreditNoteCreditNoteTypeCorrected),
        }
    }
}

/// All the variants of Enum1001 that are not matched to any variant of crate::zugferd_2_3_2::Enum1001
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ErrFromEnum1001ToCrateZugferd232Enum1001 {
    SelfBilledCorrectiveInvoiceInvoiceTypeCorrected,
    FactoredCorrectiveInvoiceInvoiceTypeCorrected,
    SelfBilledFactoredCorrectiveInvoiceInvoiceTypeCorrected,
    SelfPrepaymentInvoiceInvoiceTypeOriginal,
    SelfBilledFactoredInvoiceInvoiceTypeOriginal,
    SelfBilletFactoredCreditNoteCreditNoteTypeCorrected,
    PrepaymentCreditNoteCreditNoteTypeCorrected,
}

impl std::fmt::Display for ErrFromEnum1001ToCrateZugferd232Enum1001 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrFromEnum1001ToCrateZugferd232Enum1001::SelfBilledCorrectiveInvoiceInvoiceTypeCorrected => write!(f, "SelfBilledCorrectiveInvoiceInvoiceTypeCorrected has no corresponding value in crate::zugferd_2_3_2::Enum1001"),
            ErrFromEnum1001ToCrateZugferd232Enum1001::FactoredCorrectiveInvoiceInvoiceTypeCorrected => write!(f, "FactoredCorrectiveInvoiceInvoiceTypeCorrected has no corresponding value in crate::zugferd_2_3_2::Enum1001"),
            ErrFromEnum1001ToCrateZugferd232Enum1001::SelfBilledFactoredCorrectiveInvoiceInvoiceTypeCorrected => write!(f, "SelfBilledFactoredCorrectiveInvoiceInvoiceTypeCorrected has no corresponding value in crate::zugferd_2_3_2::Enum1001"),
            ErrFromEnum1001ToCrateZugferd232Enum1001::SelfPrepaymentInvoiceInvoiceTypeOriginal => write!(f, "SelfPrepaymentInvoiceInvoiceTypeOriginal has no corresponding value in crate::zugferd_2_3_2::Enum1001"),
            ErrFromEnum1001ToCrateZugferd232Enum1001::SelfBilledFactoredInvoiceInvoiceTypeOriginal => write!(f, "SelfBilledFactoredInvoiceInvoiceTypeOriginal has no corresponding value in crate::zugferd_2_3_2::Enum1001"),
            ErrFromEnum1001ToCrateZugferd232Enum1001::SelfBilletFactoredCreditNoteCreditNoteTypeCorrected => write!(f, "SelfBilletFactoredCreditNoteCreditNoteTypeCorrected has no corresponding value in crate::zugferd_2_3_2::Enum1001"),
            ErrFromEnum1001ToCrateZugferd232Enum1001::PrepaymentCreditNoteCreditNoteTypeCorrected => write!(f, "PrepaymentCreditNoteCreditNoteTypeCorrected has no corresponding value in crate::zugferd_2_3_2::Enum1001"),
        }
    }
}

impl std::error::Error for ErrFromEnum1001ToCrateZugferd232Enum1001 {}

impl std::convert::TryFrom<crate::zugferd_2_3_2::Enum1001> for Enum1001 {
    type Error = std::convert::Infallible;
    fn try_from(value: crate::zugferd_2_3_2::Enum1001) -> Result<Enum1001, Self::Error> {
        match value {
            crate::zugferd_2_3_2::Enum1001::RequestForPayment => Ok(Enum1001::RequestForPayment),
            crate::zugferd_2_3_2::Enum1001::DebitNoteRelatedToGoodsOrServices => {
                Ok(Enum1001::DebitNoteRelatedToGoodsOrServices)
            }
            crate::zugferd_2_3_2::Enum1001::CreditNoteRelatedToGoodsOrServices => {
                Ok(Enum1001::CreditNoteRelatedToGoodsOrServices)
            }
            crate::zugferd_2_3_2::Enum1001::MeteredServicesInvoice => {
                Ok(Enum1001::MeteredServicesInvoice)
            }
            crate::zugferd_2_3_2::Enum1001::CreditNoteRelatedToFinancialAdjustments => {
                Ok(Enum1001::CreditNoteRelatedToFinancialAdjustments)
            }
            crate::zugferd_2_3_2::Enum1001::DebitNoteRelatedToFinancialAdjustments => {
                Ok(Enum1001::DebitNoteRelatedToFinancialAdjustments)
            }
            crate::zugferd_2_3_2::Enum1001::TaxNotification => Ok(Enum1001::TaxNotification),
            crate::zugferd_2_3_2::Enum1001::InvoicingDataSheet => Ok(Enum1001::InvoicingDataSheet),
            crate::zugferd_2_3_2::Enum1001::DirectPaymentValuation => {
                Ok(Enum1001::DirectPaymentValuation)
            }
            crate::zugferd_2_3_2::Enum1001::ProvisionalPaymentValuation => {
                Ok(Enum1001::ProvisionalPaymentValuation)
            }
            crate::zugferd_2_3_2::Enum1001::PaymentValuation => Ok(Enum1001::PaymentValuation),
            crate::zugferd_2_3_2::Enum1001::InterimApplicationForPayment => {
                Ok(Enum1001::InterimApplicationForPayment)
            }
            crate::zugferd_2_3_2::Enum1001::FinalPaymentRequestBasedOnCompletionWork => {
                Ok(Enum1001::FinalPaymentRequestBasedOnCompletionWork)
            }
            crate::zugferd_2_3_2::Enum1001::PaymentRequestForCompletedUnits => {
                Ok(Enum1001::PaymentRequestForCompletedUnits)
            }
            crate::zugferd_2_3_2::Enum1001::SelfBilledCreditNote => {
                Ok(Enum1001::SelfBilledCreditNote)
            }
            crate::zugferd_2_3_2::Enum1001::ConsolidatedCreditNoteGoodsAndServices => {
                Ok(Enum1001::ConsolidatedCreditNoteGoodsAndServices)
            }
            crate::zugferd_2_3_2::Enum1001::PriceVariationInvoice => {
                Ok(Enum1001::PriceVariationInvoice)
            }
            crate::zugferd_2_3_2::Enum1001::CreditNoteForPriceVariation => {
                Ok(Enum1001::CreditNoteForPriceVariation)
            }
            crate::zugferd_2_3_2::Enum1001::DelcredereCreditNote => {
                Ok(Enum1001::DelcredereCreditNote)
            }
            crate::zugferd_2_3_2::Enum1001::ProformaInvoice => Ok(Enum1001::ProformaInvoice),
            crate::zugferd_2_3_2::Enum1001::PartialInvoice => Ok(Enum1001::PartialInvoice),
            crate::zugferd_2_3_2::Enum1001::CommercialInvoiceWhichIncludesAPackingList => {
                Ok(Enum1001::CommercialInvoiceWhichIncludesAPackingList)
            }
            crate::zugferd_2_3_2::Enum1001::CommercialInvoice => Ok(Enum1001::CommercialInvoice),
            crate::zugferd_2_3_2::Enum1001::CreditNote => Ok(Enum1001::CreditNote),
            crate::zugferd_2_3_2::Enum1001::CommissionNote => Ok(Enum1001::CommissionNote),
            crate::zugferd_2_3_2::Enum1001::DebitNote => Ok(Enum1001::DebitNote),
            crate::zugferd_2_3_2::Enum1001::CorrectedInvoice => Ok(Enum1001::CorrectedInvoice),
            crate::zugferd_2_3_2::Enum1001::ConsolidatedInvoice => {
                Ok(Enum1001::ConsolidatedInvoice)
            }
            crate::zugferd_2_3_2::Enum1001::PrepaymentInvoice => Ok(Enum1001::PrepaymentInvoice),
            crate::zugferd_2_3_2::Enum1001::HireInvoice => Ok(Enum1001::HireInvoice),
            crate::zugferd_2_3_2::Enum1001::TaxInvoice => Ok(Enum1001::TaxInvoice),
            crate::zugferd_2_3_2::Enum1001::SelfBilledInvoice => Ok(Enum1001::SelfBilledInvoice),
            crate::zugferd_2_3_2::Enum1001::DelcredereInvoice => Ok(Enum1001::DelcredereInvoice),
            crate::zugferd_2_3_2::Enum1001::FactoredInvoice => Ok(Enum1001::FactoredInvoice),
            crate::zugferd_2_3_2::Enum1001::LeaseInvoice => Ok(Enum1001::LeaseInvoice),
            crate::zugferd_2_3_2::Enum1001::ConsignmentInvoice => Ok(Enum1001::ConsignmentInvoice),
            crate::zugferd_2_3_2::Enum1001::FactoredCreditNote => Ok(Enum1001::FactoredCreditNote),
            crate::zugferd_2_3_2::Enum1001::OpticalCharacterReadingOcrPaymentCreditNote => {
                Ok(Enum1001::OpticalCharacterReadingOcrPaymentCreditNote)
            }
            crate::zugferd_2_3_2::Enum1001::DebitAdvice => Ok(Enum1001::DebitAdvice),
            crate::zugferd_2_3_2::Enum1001::ReversalDebit => Ok(Enum1001::ReversalDebit),
            crate::zugferd_2_3_2::Enum1001::ReversalCredit => Ok(Enum1001::ReversalCredit),
            crate::zugferd_2_3_2::Enum1001::SelfBilledDebitNote => {
                Ok(Enum1001::SelfBilledDebitNote)
            }
            crate::zugferd_2_3_2::Enum1001::ForwardersCreditNote => {
                Ok(Enum1001::ForwardersCreditNote)
            }
            crate::zugferd_2_3_2::Enum1001::ForwardersInvoiceDiscrepancyReport => {
                Ok(Enum1001::ForwardersInvoiceDiscrepancyReport)
            }
            crate::zugferd_2_3_2::Enum1001::InsurersInvoice => Ok(Enum1001::InsurersInvoice),
            crate::zugferd_2_3_2::Enum1001::ForwardersInvoice => Ok(Enum1001::ForwardersInvoice),
            crate::zugferd_2_3_2::Enum1001::PortChargesDocuments => {
                Ok(Enum1001::PortChargesDocuments)
            }
            crate::zugferd_2_3_2::Enum1001::InvoiceInformationForAccountingPurposes => {
                Ok(Enum1001::InvoiceInformationForAccountingPurposes)
            }
            crate::zugferd_2_3_2::Enum1001::FreightInvoice => Ok(Enum1001::FreightInvoice),
            crate::zugferd_2_3_2::Enum1001::ClaimNotification => Ok(Enum1001::ClaimNotification),
            crate::zugferd_2_3_2::Enum1001::ConsularInvoice => Ok(Enum1001::ConsularInvoice),
            crate::zugferd_2_3_2::Enum1001::PartialConstructionInvoice => {
                Ok(Enum1001::PartialConstructionInvoice)
            }
            crate::zugferd_2_3_2::Enum1001::PartialFinalConstructionInvoice => {
                Ok(Enum1001::PartialFinalConstructionInvoice)
            }
            crate::zugferd_2_3_2::Enum1001::FinalConstructionInvoice => {
                Ok(Enum1001::FinalConstructionInvoice)
            }
            crate::zugferd_2_3_2::Enum1001::CustomsInvoice => Ok(Enum1001::CustomsInvoice),
        }
    }
}
// End: (Version) TryFrom crate::zugferd_2_3_2::Enum1001 to Enum1001
