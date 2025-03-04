#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum LineStatus {
    /// Added
    Added,
    /// Deleted
    Deleted,
    /// Changed
    Changed,
    /// No action
    NoAction,
    /// Accepted without amendment
    AcceptedWithoutAmendment,
    /// Accepted with amendment
    AcceptedWithAmendment,
    /// Not accepted
    NotAccepted,
    /// Schedule only
    ScheduleOnly,
    /// Amendments
    Amendments,
    /// Not found
    NotFound,
    /// Not amended
    NotAmended,
    /// Line item numbers changed
    LineItemNumbersChanged,
    /// Buyer has deducted amount
    BuyerHasDeductedAmount,
    /// Buyer claims against invoice
    BuyerClaimsAgainstInvoice,
    /// Charge back by seller
    ChargeBackBySeller,
    /// Seller will issue credit note
    SellerWillIssueCreditNote,
    /// Terms changed for new terms
    TermsChangedForNewTerms,
    /// Abide outcome of negotiations
    AbideOutcomeNegotiations,
    /// Seller rejects dispute
    SellerRejectsDispute,
    /// Settlement
    Settlement,
    /// No delivery
    NoDelivery,
    /// Call-off delivery
    CallOffDelivery,
    /// Proposed amendment
    ProposedAmendment,
    /// Accepted with amendment, no confirmation required
    AcceptedWithAmendmentNoConfirmationRequired,
    /// Equipment provisionally repaired
    EquipmentProvisionallyRepaired,
    /// Included
    Included,
    /// Upon receipt and verification of documents we shall cover you when due as per your instructions
    UponReceiptAndVerificationDocumentsWeShallCoverYouWhenDueAsPerYourInstructions,
    /// Upon receipt and verification of documents we shall authorize you to debit our account with you when due
    UponReceiptAndVerificationDocumentsWeShallAuthorizeYouToDebitOurAccountWithYouWhenDue,
    /// On receipt of your authenticated advice we shall cover you when due as per your instructions
    OnReceiptYourAuthenticatedAdviceWeShallCoverYouWhenDueAsPerYourInstructions,
    /// On receipt of your authenticated advice we shall authorize you to debit our account with you when due
    OnReceiptYourAuthenticatedAdviceWeShallAuthorizeYouToDebitOurAccountWithYouWhenDue,
    /// On receipt of your authenticated advice we shall credit your account with us when due
    OnReceiptYourAuthenticatedAdviceWeShallCreditYourAccountWithUsWhenDue,
    /// Credit advice requested for direct debit
    CreditAdviceRequestedForDirectDebit,
    /// Credit advice and acknowledgement for direct debit
    CreditAdviceAndAcknowledgementForDirectDebit,
    /// Inquiry
    Inquiry,
    /// Checked
    Checked,
    /// Not checked
    NotChecked,
    /// Cancelled
    Cancelled,
    /// Replaced
    Replaced,
    /// New
    New,
    /// Agreed
    Agreed,
    /// Proposed
    Proposed,
    /// Already delivered
    AlreadyDelivered,
    /// Additional subordinate structures will follow
    AdditionalSubordinateStructuresWillFollow,
    /// Additional subordinate structures will not follow
    AdditionalSubordinateStructuresWillNotFollow,
    /// Result opposed
    ResultOpposed,
    /// Auction held
    AuctionHeld,
    /// Legal action pursued
    LegalActionPursued,
    /// Meeting held
    MeetingHeld,
    /// Result set aside
    ResultSetAside,
    /// Result disputed
    ResultDisputed,
    /// Countersued
    Countersued,
    /// Pending
    Pending,
    /// Court action dismissed
    CourtActionDismissed,
    /// Referred item, accepted
    ReferredItemAccepted,
    /// Referred item, rejected
    ReferredItemRejected,
    /// Debit advice statement line
    DebitAdviceStatementLine,
    /// Credit advice statement line
    CreditAdviceStatementLine,
    /// Grouped credit advices
    GroupedCreditAdvices,
    /// Grouped debit advices
    GroupedDebitAdvices,
    /// Registered
    Registered,
    /// Payment denied
    PaymentDenied,
    /// Approved as amended
    ApprovedAsAmended,
    /// Approved as submitted
    ApprovedAsSubmitted,
    /// Cancelled, no activity
    CancelledNoActivity,
    /// Under investigation
    UnderInvestigation,
    /// Initial claim received
    InitialClaimReceived,
    /// Not in process
    NotInProcess,
    /// Rejected, duplicate
    RejectedDuplicate,
    /// Rejected, resubmit with corrections
    RejectedResubmitWithCorrections,
    /// Pending, incomplete
    PendingIncomplete,
    /// Under field office investigation
    UnderFieldOfficeInvestigation,
    /// Pending, awaiting additional material
    PendingAwaitingAdditionalMaterial,
    /// Pending, awaiting review
    PendingAwaitingReview,
    /// Reopened
    Reopened,
    /// Processed by primary, forwarded to additional payer(s)
    ProcessedByPrimaryForwardedToAdditionalPayerS,
    /// Processed by secondary, forwarded to additional payer(s)
    ProcessedBySecondaryForwardedToAdditionalPayerS,
    /// Processed by tertiary, forwarded to additional payer(s)
    ProcessedByTertiaryForwardedToAdditionalPayerS,
    /// Previous payment decision reversed
    PreviousPaymentDecisionReversed,
    /// Not our claim, forwarded to another payer(s)
    NotOurClaimForwardedToAnotherPayerS,
    /// Transferred to correct insurance carrier
    TransferredToCorrectInsuranceCarrier,
    /// Not paid, predetermination pricing only
    NotPaidPredeterminationPricingOnly,
    /// Documentation claim
    DocumentationClaim,
    /// Reviewed
    Reviewed,
    /// Repriced
    Repriced,
    /// Audited
    Audited,
    /// Conditionally paid
    ConditionallyPaid,
    /// On appeal
    OnAppeal,
    /// Closed
    Closed,
    /// Reaudited
    Reaudited,
    /// Reissued
    Reissued,
    /// Closed after reopening
    ClosedAfterReopening,
    /// Redetermined
    Redetermined,
    /// Processed as primary
    ProcessedAsPrimary,
    /// Processed as secondary
    ProcessedAsSecondary,
    /// Processed as tertiary
    ProcessedAsTertiary,
    /// Correction of error
    CorrectionError,
    /// Single credit item of a group
    SingleCreditItemAGroup,
    /// Single debit item of a group
    SingleDebitItemAGroup,
    /// Interim response
    InterimResponse,
    /// Final response
    FinalResponse,
    /// Debit advice requested
    DebitAdviceRequested,
    /// Transaction not impacted
    TransactionNotImpacted,
    /// Patient to be notified
    PatientToBeNotified,
    /// Healthcare provider to be notified
    HealthcareProviderToBeNotified,
    /// Usual general practitioner to be notified
    UsualGeneralPractitionerToBeNotified,
    /// Advice without details
    AdviceWithoutDetails,
    /// Advice with details
    AdviceWithDetails,
    /// Amendment requested
    AmendmentRequested,
    /// For information
    ForInformation,
    /// Withdraw
    Withdraw,
    /// Delivery date change
    DeliveryDateChange,
    /// Quantity change
    QuantityChange,
    /// Resale and claim
    ResaleAndClaim,
    /// Resale
    Resale,
    /// Prior addition
    PriorAddition,
    /// Expired
    Expired,
    /// Hold
    Hold,
    /// Open
    Open,
    /// Observe
    Observe,
}

impl crate::Code for LineStatus {
    fn code(&self) -> &str {
        match self {
            LineStatus::Added => "1",
            LineStatus::Deleted => "2",
            LineStatus::Changed => "3",
            LineStatus::NoAction => "4",
            LineStatus::AcceptedWithoutAmendment => "5",
            LineStatus::AcceptedWithAmendment => "6",
            LineStatus::NotAccepted => "7",
            LineStatus::ScheduleOnly => "8",
            LineStatus::Amendments => "9",
            LineStatus::NotFound => "10",
            LineStatus::NotAmended => "11",
            LineStatus::LineItemNumbersChanged => "12",
            LineStatus::BuyerHasDeductedAmount => "13",
            LineStatus::BuyerClaimsAgainstInvoice => "14",
            LineStatus::ChargeBackBySeller => "15",
            LineStatus::SellerWillIssueCreditNote => "16",
            LineStatus::TermsChangedForNewTerms => "17",
            LineStatus::AbideOutcomeNegotiations => "18",
            LineStatus::SellerRejectsDispute => "19",
            LineStatus::Settlement => "20",
            LineStatus::NoDelivery => "21",
            LineStatus::CallOffDelivery => "22",
            LineStatus::ProposedAmendment => "23",
            LineStatus::AcceptedWithAmendmentNoConfirmationRequired => "24",
            LineStatus::EquipmentProvisionallyRepaired => "25",
            LineStatus::Included => "26",
            LineStatus::UponReceiptAndVerificationDocumentsWeShallCoverYouWhenDueAsPerYourInstructions => "27",
            LineStatus::UponReceiptAndVerificationDocumentsWeShallAuthorizeYouToDebitOurAccountWithYouWhenDue => "28",
            LineStatus::OnReceiptYourAuthenticatedAdviceWeShallCoverYouWhenDueAsPerYourInstructions => "29",
            LineStatus::OnReceiptYourAuthenticatedAdviceWeShallAuthorizeYouToDebitOurAccountWithYouWhenDue => "30",
            LineStatus::OnReceiptYourAuthenticatedAdviceWeShallCreditYourAccountWithUsWhenDue => "31",
            LineStatus::CreditAdviceRequestedForDirectDebit => "32",
            LineStatus::CreditAdviceAndAcknowledgementForDirectDebit => "33",
            LineStatus::Inquiry => "34",
            LineStatus::Checked => "35",
            LineStatus::NotChecked => "36",
            LineStatus::Cancelled => "37",
            LineStatus::Replaced => "38",
            LineStatus::New => "39",
            LineStatus::Agreed => "40",
            LineStatus::Proposed => "41",
            LineStatus::AlreadyDelivered => "42",
            LineStatus::AdditionalSubordinateStructuresWillFollow => "43",
            LineStatus::AdditionalSubordinateStructuresWillNotFollow => "44",
            LineStatus::ResultOpposed => "45",
            LineStatus::AuctionHeld => "46",
            LineStatus::LegalActionPursued => "47",
            LineStatus::MeetingHeld => "48",
            LineStatus::ResultSetAside => "49",
            LineStatus::ResultDisputed => "50",
            LineStatus::Countersued => "51",
            LineStatus::Pending => "52",
            LineStatus::CourtActionDismissed => "53",
            LineStatus::ReferredItemAccepted => "54",
            LineStatus::ReferredItemRejected => "55",
            LineStatus::DebitAdviceStatementLine => "56",
            LineStatus::CreditAdviceStatementLine => "57",
            LineStatus::GroupedCreditAdvices => "58",
            LineStatus::GroupedDebitAdvices => "59",
            LineStatus::Registered => "60",
            LineStatus::PaymentDenied => "61",
            LineStatus::ApprovedAsAmended => "62",
            LineStatus::ApprovedAsSubmitted => "63",
            LineStatus::CancelledNoActivity => "64",
            LineStatus::UnderInvestigation => "65",
            LineStatus::InitialClaimReceived => "66",
            LineStatus::NotInProcess => "67",
            LineStatus::RejectedDuplicate => "68",
            LineStatus::RejectedResubmitWithCorrections => "69",
            LineStatus::PendingIncomplete => "70",
            LineStatus::UnderFieldOfficeInvestigation => "71",
            LineStatus::PendingAwaitingAdditionalMaterial => "72",
            LineStatus::PendingAwaitingReview => "73",
            LineStatus::Reopened => "74",
            LineStatus::ProcessedByPrimaryForwardedToAdditionalPayerS => "75",
            LineStatus::ProcessedBySecondaryForwardedToAdditionalPayerS => "76",
            LineStatus::ProcessedByTertiaryForwardedToAdditionalPayerS => "77",
            LineStatus::PreviousPaymentDecisionReversed => "78",
            LineStatus::NotOurClaimForwardedToAnotherPayerS => "79",
            LineStatus::TransferredToCorrectInsuranceCarrier => "80",
            LineStatus::NotPaidPredeterminationPricingOnly => "81",
            LineStatus::DocumentationClaim => "82",
            LineStatus::Reviewed => "83",
            LineStatus::Repriced => "84",
            LineStatus::Audited => "85",
            LineStatus::ConditionallyPaid => "86",
            LineStatus::OnAppeal => "87",
            LineStatus::Closed => "88",
            LineStatus::Reaudited => "89",
            LineStatus::Reissued => "90",
            LineStatus::ClosedAfterReopening => "91",
            LineStatus::Redetermined => "92",
            LineStatus::ProcessedAsPrimary => "93",
            LineStatus::ProcessedAsSecondary => "94",
            LineStatus::ProcessedAsTertiary => "95",
            LineStatus::CorrectionError => "96",
            LineStatus::SingleCreditItemAGroup => "97",
            LineStatus::SingleDebitItemAGroup => "98",
            LineStatus::InterimResponse => "99",
            LineStatus::FinalResponse => "100",
            LineStatus::DebitAdviceRequested => "101",
            LineStatus::TransactionNotImpacted => "102",
            LineStatus::PatientToBeNotified => "103",
            LineStatus::HealthcareProviderToBeNotified => "104",
            LineStatus::UsualGeneralPractitionerToBeNotified => "105",
            LineStatus::AdviceWithoutDetails => "106",
            LineStatus::AdviceWithDetails => "107",
            LineStatus::AmendmentRequested => "108",
            LineStatus::ForInformation => "109",
            LineStatus::Withdraw => "110",
            LineStatus::DeliveryDateChange => "111",
            LineStatus::QuantityChange => "112",
            LineStatus::ResaleAndClaim => "113",
            LineStatus::Resale => "114",
            LineStatus::PriorAddition => "115",
            LineStatus::Expired => "116",
            LineStatus::Hold => "117",
            LineStatus::Open => "118",
            LineStatus::Observe => "119",
        }
    }
}

impl crate::Description for LineStatus {
    fn description(&self) -> &str {
        match self {
            LineStatus::Added => "Added",
            LineStatus::Deleted => "Deleted",
            LineStatus::Changed => "Changed",
            LineStatus::NoAction => "No action",
            LineStatus::AcceptedWithoutAmendment => "Accepted without amendment",
            LineStatus::AcceptedWithAmendment => "Accepted with amendment",
            LineStatus::NotAccepted => "Not accepted",
            LineStatus::ScheduleOnly => "Schedule only",
            LineStatus::Amendments => "Amendments",
            LineStatus::NotFound => "Not found",
            LineStatus::NotAmended => "Not amended",
            LineStatus::LineItemNumbersChanged => "Line item numbers changed",
            LineStatus::BuyerHasDeductedAmount => "Buyer has deducted amount",
            LineStatus::BuyerClaimsAgainstInvoice => "Buyer claims against invoice",
            LineStatus::ChargeBackBySeller => "Charge back by seller",
            LineStatus::SellerWillIssueCreditNote => "Seller will issue credit note",
            LineStatus::TermsChangedForNewTerms => "Terms changed for new terms",
            LineStatus::AbideOutcomeNegotiations => "Abide outcome of negotiations",
            LineStatus::SellerRejectsDispute => "Seller rejects dispute",
            LineStatus::Settlement => "Settlement",
            LineStatus::NoDelivery => "No delivery",
            LineStatus::CallOffDelivery => "Call-off delivery",
            LineStatus::ProposedAmendment => "Proposed amendment",
            LineStatus::AcceptedWithAmendmentNoConfirmationRequired => "Accepted with amendment, no confirmation required",
            LineStatus::EquipmentProvisionallyRepaired => "Equipment provisionally repaired",
            LineStatus::Included => "Included",
            LineStatus::UponReceiptAndVerificationDocumentsWeShallCoverYouWhenDueAsPerYourInstructions => "Upon receipt and verification of documents we shall cover you when due as per your instructions",
            LineStatus::UponReceiptAndVerificationDocumentsWeShallAuthorizeYouToDebitOurAccountWithYouWhenDue => "Upon receipt and verification of documents we shall authorize you to debit our account with you when due",
            LineStatus::OnReceiptYourAuthenticatedAdviceWeShallCoverYouWhenDueAsPerYourInstructions => "On receipt of your authenticated advice we shall cover you when due as per your instructions",
            LineStatus::OnReceiptYourAuthenticatedAdviceWeShallAuthorizeYouToDebitOurAccountWithYouWhenDue => "On receipt of your authenticated advice we shall authorize you to debit our account with you when due",
            LineStatus::OnReceiptYourAuthenticatedAdviceWeShallCreditYourAccountWithUsWhenDue => "On receipt of your authenticated advice we shall credit your account with us when due",
            LineStatus::CreditAdviceRequestedForDirectDebit => "Credit advice requested for direct debit",
            LineStatus::CreditAdviceAndAcknowledgementForDirectDebit => "Credit advice and acknowledgement for direct debit",
            LineStatus::Inquiry => "Inquiry",
            LineStatus::Checked => "Checked",
            LineStatus::NotChecked => "Not checked",
            LineStatus::Cancelled => "Cancelled",
            LineStatus::Replaced => "Replaced",
            LineStatus::New => "New",
            LineStatus::Agreed => "Agreed",
            LineStatus::Proposed => "Proposed",
            LineStatus::AlreadyDelivered => "Already delivered",
            LineStatus::AdditionalSubordinateStructuresWillFollow => "Additional subordinate structures will follow",
            LineStatus::AdditionalSubordinateStructuresWillNotFollow => "Additional subordinate structures will not follow",
            LineStatus::ResultOpposed => "Result opposed",
            LineStatus::AuctionHeld => "Auction held",
            LineStatus::LegalActionPursued => "Legal action pursued",
            LineStatus::MeetingHeld => "Meeting held",
            LineStatus::ResultSetAside => "Result set aside",
            LineStatus::ResultDisputed => "Result disputed",
            LineStatus::Countersued => "Countersued",
            LineStatus::Pending => "Pending",
            LineStatus::CourtActionDismissed => "Court action dismissed",
            LineStatus::ReferredItemAccepted => "Referred item, accepted",
            LineStatus::ReferredItemRejected => "Referred item, rejected",
            LineStatus::DebitAdviceStatementLine => "Debit advice statement line",
            LineStatus::CreditAdviceStatementLine => "Credit advice statement line",
            LineStatus::GroupedCreditAdvices => "Grouped credit advices",
            LineStatus::GroupedDebitAdvices => "Grouped debit advices",
            LineStatus::Registered => "Registered",
            LineStatus::PaymentDenied => "Payment denied",
            LineStatus::ApprovedAsAmended => "Approved as amended",
            LineStatus::ApprovedAsSubmitted => "Approved as submitted",
            LineStatus::CancelledNoActivity => "Cancelled, no activity",
            LineStatus::UnderInvestigation => "Under investigation",
            LineStatus::InitialClaimReceived => "Initial claim received",
            LineStatus::NotInProcess => "Not in process",
            LineStatus::RejectedDuplicate => "Rejected, duplicate",
            LineStatus::RejectedResubmitWithCorrections => "Rejected, resubmit with corrections",
            LineStatus::PendingIncomplete => "Pending, incomplete",
            LineStatus::UnderFieldOfficeInvestigation => "Under field office investigation",
            LineStatus::PendingAwaitingAdditionalMaterial => "Pending, awaiting additional material",
            LineStatus::PendingAwaitingReview => "Pending, awaiting review",
            LineStatus::Reopened => "Reopened",
            LineStatus::ProcessedByPrimaryForwardedToAdditionalPayerS => "Processed by primary, forwarded to additional payer(s)",
            LineStatus::ProcessedBySecondaryForwardedToAdditionalPayerS => "Processed by secondary, forwarded to additional payer(s)",
            LineStatus::ProcessedByTertiaryForwardedToAdditionalPayerS => "Processed by tertiary, forwarded to additional payer(s)",
            LineStatus::PreviousPaymentDecisionReversed => "Previous payment decision reversed",
            LineStatus::NotOurClaimForwardedToAnotherPayerS => "Not our claim, forwarded to another payer(s)",
            LineStatus::TransferredToCorrectInsuranceCarrier => "Transferred to correct insurance carrier",
            LineStatus::NotPaidPredeterminationPricingOnly => "Not paid, predetermination pricing only",
            LineStatus::DocumentationClaim => "Documentation claim",
            LineStatus::Reviewed => "Reviewed",
            LineStatus::Repriced => "Repriced",
            LineStatus::Audited => "Audited",
            LineStatus::ConditionallyPaid => "Conditionally paid",
            LineStatus::OnAppeal => "On appeal",
            LineStatus::Closed => "Closed",
            LineStatus::Reaudited => "Reaudited",
            LineStatus::Reissued => "Reissued",
            LineStatus::ClosedAfterReopening => "Closed after reopening",
            LineStatus::Redetermined => "Redetermined",
            LineStatus::ProcessedAsPrimary => "Processed as primary",
            LineStatus::ProcessedAsSecondary => "Processed as secondary",
            LineStatus::ProcessedAsTertiary => "Processed as tertiary",
            LineStatus::CorrectionError => "Correction of error",
            LineStatus::SingleCreditItemAGroup => "Single credit item of a group",
            LineStatus::SingleDebitItemAGroup => "Single debit item of a group",
            LineStatus::InterimResponse => "Interim response",
            LineStatus::FinalResponse => "Final response",
            LineStatus::DebitAdviceRequested => "Debit advice requested",
            LineStatus::TransactionNotImpacted => "Transaction not impacted",
            LineStatus::PatientToBeNotified => "Patient to be notified",
            LineStatus::HealthcareProviderToBeNotified => "Healthcare provider to be notified",
            LineStatus::UsualGeneralPractitionerToBeNotified => "Usual general practitioner to be notified",
            LineStatus::AdviceWithoutDetails => "Advice without details",
            LineStatus::AdviceWithDetails => "Advice with details",
            LineStatus::AmendmentRequested => "Amendment requested",
            LineStatus::ForInformation => "For information",
            LineStatus::Withdraw => "Withdraw",
            LineStatus::DeliveryDateChange => "Delivery date change",
            LineStatus::QuantityChange => "Quantity change",
            LineStatus::ResaleAndClaim => "Resale and claim",
            LineStatus::Resale => "Resale",
            LineStatus::PriorAddition => "Prior addition",
            LineStatus::Expired => "Expired",
            LineStatus::Hold => "Hold",
            LineStatus::Open => "Open",
            LineStatus::Observe => "Observe",
        }
    }
}

impl crate::FromCode for LineStatus {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "1" => Some(LineStatus::Added),
            "2" => Some(LineStatus::Deleted),
            "3" => Some(LineStatus::Changed),
            "4" => Some(LineStatus::NoAction),
            "5" => Some(LineStatus::AcceptedWithoutAmendment),
            "6" => Some(LineStatus::AcceptedWithAmendment),
            "7" => Some(LineStatus::NotAccepted),
            "8" => Some(LineStatus::ScheduleOnly),
            "9" => Some(LineStatus::Amendments),
            "10" => Some(LineStatus::NotFound),
            "11" => Some(LineStatus::NotAmended),
            "12" => Some(LineStatus::LineItemNumbersChanged),
            "13" => Some(LineStatus::BuyerHasDeductedAmount),
            "14" => Some(LineStatus::BuyerClaimsAgainstInvoice),
            "15" => Some(LineStatus::ChargeBackBySeller),
            "16" => Some(LineStatus::SellerWillIssueCreditNote),
            "17" => Some(LineStatus::TermsChangedForNewTerms),
            "18" => Some(LineStatus::AbideOutcomeNegotiations),
            "19" => Some(LineStatus::SellerRejectsDispute),
            "20" => Some(LineStatus::Settlement),
            "21" => Some(LineStatus::NoDelivery),
            "22" => Some(LineStatus::CallOffDelivery),
            "23" => Some(LineStatus::ProposedAmendment),
            "24" => Some(LineStatus::AcceptedWithAmendmentNoConfirmationRequired),
            "25" => Some(LineStatus::EquipmentProvisionallyRepaired),
            "26" => Some(LineStatus::Included),
            "27" => Some(LineStatus::UponReceiptAndVerificationDocumentsWeShallCoverYouWhenDueAsPerYourInstructions),
            "28" => Some(LineStatus::UponReceiptAndVerificationDocumentsWeShallAuthorizeYouToDebitOurAccountWithYouWhenDue),
            "29" => Some(LineStatus::OnReceiptYourAuthenticatedAdviceWeShallCoverYouWhenDueAsPerYourInstructions),
            "30" => Some(LineStatus::OnReceiptYourAuthenticatedAdviceWeShallAuthorizeYouToDebitOurAccountWithYouWhenDue),
            "31" => Some(LineStatus::OnReceiptYourAuthenticatedAdviceWeShallCreditYourAccountWithUsWhenDue),
            "32" => Some(LineStatus::CreditAdviceRequestedForDirectDebit),
            "33" => Some(LineStatus::CreditAdviceAndAcknowledgementForDirectDebit),
            "34" => Some(LineStatus::Inquiry),
            "35" => Some(LineStatus::Checked),
            "36" => Some(LineStatus::NotChecked),
            "37" => Some(LineStatus::Cancelled),
            "38" => Some(LineStatus::Replaced),
            "39" => Some(LineStatus::New),
            "40" => Some(LineStatus::Agreed),
            "41" => Some(LineStatus::Proposed),
            "42" => Some(LineStatus::AlreadyDelivered),
            "43" => Some(LineStatus::AdditionalSubordinateStructuresWillFollow),
            "44" => Some(LineStatus::AdditionalSubordinateStructuresWillNotFollow),
            "45" => Some(LineStatus::ResultOpposed),
            "46" => Some(LineStatus::AuctionHeld),
            "47" => Some(LineStatus::LegalActionPursued),
            "48" => Some(LineStatus::MeetingHeld),
            "49" => Some(LineStatus::ResultSetAside),
            "50" => Some(LineStatus::ResultDisputed),
            "51" => Some(LineStatus::Countersued),
            "52" => Some(LineStatus::Pending),
            "53" => Some(LineStatus::CourtActionDismissed),
            "54" => Some(LineStatus::ReferredItemAccepted),
            "55" => Some(LineStatus::ReferredItemRejected),
            "56" => Some(LineStatus::DebitAdviceStatementLine),
            "57" => Some(LineStatus::CreditAdviceStatementLine),
            "58" => Some(LineStatus::GroupedCreditAdvices),
            "59" => Some(LineStatus::GroupedDebitAdvices),
            "60" => Some(LineStatus::Registered),
            "61" => Some(LineStatus::PaymentDenied),
            "62" => Some(LineStatus::ApprovedAsAmended),
            "63" => Some(LineStatus::ApprovedAsSubmitted),
            "64" => Some(LineStatus::CancelledNoActivity),
            "65" => Some(LineStatus::UnderInvestigation),
            "66" => Some(LineStatus::InitialClaimReceived),
            "67" => Some(LineStatus::NotInProcess),
            "68" => Some(LineStatus::RejectedDuplicate),
            "69" => Some(LineStatus::RejectedResubmitWithCorrections),
            "70" => Some(LineStatus::PendingIncomplete),
            "71" => Some(LineStatus::UnderFieldOfficeInvestigation),
            "72" => Some(LineStatus::PendingAwaitingAdditionalMaterial),
            "73" => Some(LineStatus::PendingAwaitingReview),
            "74" => Some(LineStatus::Reopened),
            "75" => Some(LineStatus::ProcessedByPrimaryForwardedToAdditionalPayerS),
            "76" => Some(LineStatus::ProcessedBySecondaryForwardedToAdditionalPayerS),
            "77" => Some(LineStatus::ProcessedByTertiaryForwardedToAdditionalPayerS),
            "78" => Some(LineStatus::PreviousPaymentDecisionReversed),
            "79" => Some(LineStatus::NotOurClaimForwardedToAnotherPayerS),
            "80" => Some(LineStatus::TransferredToCorrectInsuranceCarrier),
            "81" => Some(LineStatus::NotPaidPredeterminationPricingOnly),
            "82" => Some(LineStatus::DocumentationClaim),
            "83" => Some(LineStatus::Reviewed),
            "84" => Some(LineStatus::Repriced),
            "85" => Some(LineStatus::Audited),
            "86" => Some(LineStatus::ConditionallyPaid),
            "87" => Some(LineStatus::OnAppeal),
            "88" => Some(LineStatus::Closed),
            "89" => Some(LineStatus::Reaudited),
            "90" => Some(LineStatus::Reissued),
            "91" => Some(LineStatus::ClosedAfterReopening),
            "92" => Some(LineStatus::Redetermined),
            "93" => Some(LineStatus::ProcessedAsPrimary),
            "94" => Some(LineStatus::ProcessedAsSecondary),
            "95" => Some(LineStatus::ProcessedAsTertiary),
            "96" => Some(LineStatus::CorrectionError),
            "97" => Some(LineStatus::SingleCreditItemAGroup),
            "98" => Some(LineStatus::SingleDebitItemAGroup),
            "99" => Some(LineStatus::InterimResponse),
            "100" => Some(LineStatus::FinalResponse),
            "101" => Some(LineStatus::DebitAdviceRequested),
            "102" => Some(LineStatus::TransactionNotImpacted),
            "103" => Some(LineStatus::PatientToBeNotified),
            "104" => Some(LineStatus::HealthcareProviderToBeNotified),
            "105" => Some(LineStatus::UsualGeneralPractitionerToBeNotified),
            "106" => Some(LineStatus::AdviceWithoutDetails),
            "107" => Some(LineStatus::AdviceWithDetails),
            "108" => Some(LineStatus::AmendmentRequested),
            "109" => Some(LineStatus::ForInformation),
            "110" => Some(LineStatus::Withdraw),
            "111" => Some(LineStatus::DeliveryDateChange),
            "112" => Some(LineStatus::QuantityChange),
            "113" => Some(LineStatus::ResaleAndClaim),
            "114" => Some(LineStatus::Resale),
            "115" => Some(LineStatus::PriorAddition),
            "116" => Some(LineStatus::Expired),
            "117" => Some(LineStatus::Hold),
            "118" => Some(LineStatus::Open),
            "119" => Some(LineStatus::Observe),
            _ => None,
        }
    }
}
