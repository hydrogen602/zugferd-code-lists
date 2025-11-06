export enum LineStatus {
  /**
   * Added
   *
   * The information is to be or has been added.
   */
  Added = "1",
  /**
   * Deleted
   *
   * The information is to be or has been deleted.
   */
  Deleted = "2",
  /**
   * Changed
   *
   * The information is to be or has been changed.
   */
  Changed = "3",
  /**
   * No action
   *
   * This line item is not affected by the actual message.
   */
  NoAction = "4",
  /**
   * Accepted without amendment
   *
   * This line item is entirely accepted by the seller.
   */
  AcceptedWithoutAmendment = "5",
  /**
   * Accepted with amendment
   *
   * This line item is accepted but amended by the seller.
   */
  AcceptedWithAmendment = "6",
  /**
   * Not accepted
   *
   * This line item is not accepted by the seller.
   */
  NotAccepted = "7",
  /**
   * Schedule only
   *
   * Code specifying that the message is a schedule only.
   */
  ScheduleOnly = "8",
  /**
   * Amendments
   *
   * Code specifying that amendments are requested/notified.
   */
  Amendments = "9",
  /**
   * Not found
   *
   * This line item is not found in the referenced message.
   */
  NotFound = "10",
  /**
   * Not amended
   *
   * This line is not amended by the buyer.
   */
  NotAmended = "11",
  /**
   * Line item numbers changed
   *
   * Code specifying that the line item numbers have changed.
   */
  LineItemNumbersChanged = "12",
  /**
   * Buyer has deducted amount
   *
   * Buyer has deducted amount from payment.
   */
  BuyerHasDeductedAmount = "13",
  /**
   * Buyer claims against invoice
   *
   * Buyer has a claim against an outstanding invoice.
   */
  BuyerClaimsAgainstInvoice = "14",
  /**
   * Charge back by seller
   *
   * Factor has been requested to charge back the outstanding item.
   */
  ChargeBackBySeller = "15",
  /**
   * Seller will issue credit note
   *
   * Seller agrees to issue a credit note.
   */
  SellerWillIssueCreditNote = "16",
  /**
   * Terms changed for new terms
   *
   * New settlement terms have been agreed.
   */
  TermsChangedForNewTerms = "17",
  /**
   * Abide outcome of negotiations
   *
   * Factor agrees to abide by the outcome of negotiations between seller and buyer.
   */
  AbideOutcomeNegotiations = "18",
  /**
   * Seller rejects dispute
   *
   * Seller does not accept validity of dispute.
   */
  SellerRejectsDispute = "19",
  /**
   * Settlement
   *
   * The reported situation is settled.
   */
  Settlement = "20",
  /**
   * No delivery
   *
   * Code indicating that no delivery will be required.
   */
  NoDelivery = "21",
  /**
   * Call-off delivery
   *
   * A request for delivery of a particular quantity of goods to be delivered on a particular date (or within a particular period).
   */
  CallOffDelivery = "22",
  /**
   * Proposed amendment
   *
   * A code used to indicate an amendment suggested by the sender.
   */
  ProposedAmendment = "23",
  /**
   * Accepted with amendment, no confirmation required
   *
   * Accepted with changes which require no confirmation.
   */
  AcceptedWithAmendmentNoConfirmationRequired = "24",
  /**
   * Equipment provisionally repaired
   *
   * The equipment or component has been provisionally repaired.
   */
  EquipmentProvisionallyRepaired = "25",
  /**
   * Included
   *
   * Code indicating that the entity is included.
   */
  Included = "26",
  /**
   * Upon receipt and verification of documents we shall cover you when due as per your instructions
   *
   * Upon receipt and verification of documents we shall cover you when due as per your instructions.
   */
  UponReceiptAndVerificationDocumentsWeShallCoverYouWhenDueAsPerYourInstructions = "27",
  /**
   * Upon receipt and verification of documents we shall authorize you to debit our account with you when due
   *
   * Upon receipt and verification of documents we shall authorize you to debit our account with you when due.
   */
  UponReceiptAndVerificationDocumentsWeShallAuthorizeYouToDebitOurAccountWithYouWhenDue = "28",
  /**
   * On receipt of your authenticated advice we shall cover you when due as per your instructions
   *
   * On receipt of your authenticated advice we shall cover you when due as per your instructions.
   */
  OnReceiptYourAuthenticatedAdviceWeShallCoverYouWhenDueAsPerYourInstructions = "29",
  /**
   * On receipt of your authenticated advice we shall authorize you to debit our account with you when due
   *
   * On receipt of your authenticated advice we shall authorize you to debit our account with you when due.
   */
  OnReceiptYourAuthenticatedAdviceWeShallAuthorizeYouToDebitOurAccountWithYouWhenDue = "30",
  /**
   * On receipt of your authenticated advice we shall credit your account with us when due
   *
   * On receipt of your authenticated advice we shall credit your account with us when due.
   */
  OnReceiptYourAuthenticatedAdviceWeShallCreditYourAccountWithUsWhenDue = "31",
  /**
   * Credit advice requested for direct debit
   *
   * A credit advice is requested for the direct debit.
   */
  CreditAdviceRequestedForDirectDebit = "32",
  /**
   * Credit advice and acknowledgement for direct debit
   *
   * A credit advice and acknowledgement are requested for the direct debit.
   */
  CreditAdviceAndAcknowledgementForDirectDebit = "33",
  /**
   * Inquiry
   *
   * Request for information.
   */
  Inquiry = "34",
  /**
   * Checked
   *
   * Checked.
   */
  Checked = "35",
  /**
   * Not checked
   *
   * Not checked.
   */
  NotChecked = "36",
  /**
   * Cancelled
   *
   * Discontinued.
   */
  Cancelled = "37",
  /**
   * Replaced
   *
   * Provide a replacement.
   */
  Replaced = "38",
  /**
   * New
   *
   * Not existing before.
   */
  New = "39",
  /**
   * Agreed
   *
   * Consent.
   */
  Agreed = "40",
  /**
   * Proposed
   *
   * Put forward for consideration.
   */
  Proposed = "41",
  /**
   * Already delivered
   *
   * Delivery has taken place.
   */
  AlreadyDelivered = "42",
  /**
   * Additional subordinate structures will follow
   *
   * Additional subordinate structures will follow the current hierarchy level.
   */
  AdditionalSubordinateStructuresWillFollow = "43",
  /**
   * Additional subordinate structures will not follow
   *
   * No additional subordinate structures will follow the current hierarchy level.
   */
  AdditionalSubordinateStructuresWillNotFollow = "44",
  /**
   * Result opposed
   *
   * A notification that the result is opposed.
   */
  ResultOpposed = "45",
  /**
   * Auction held
   *
   * A notification that an auction was held.
   */
  AuctionHeld = "46",
  /**
   * Legal action pursued
   *
   * A notification that legal action has been pursued.
   */
  LegalActionPursued = "47",
  /**
   * Meeting held
   *
   * A notification that a meeting was held.
   */
  MeetingHeld = "48",
  /**
   * Result set aside
   *
   * A notification that the result has been set aside.
   */
  ResultSetAside = "49",
  /**
   * Result disputed
   *
   * A notification that the result has been disputed.
   */
  ResultDisputed = "50",
  /**
   * Countersued
   *
   * A notification that a countersuit has been filed.
   */
  Countersued = "51",
  /**
   * Pending
   *
   * A notification that an action is awaiting settlement.
   */
  Pending = "52",
  /**
   * Court action dismissed
   *
   * A notification that a court action will no longer be heard.
   */
  CourtActionDismissed = "53",
  /**
   * Referred item, accepted
   *
   * The item being referred to has been accepted.
   */
  ReferredItemAccepted = "54",
  /**
   * Referred item, rejected
   *
   * The item being referred to has been rejected.
   */
  ReferredItemRejected = "55",
  /**
   * Debit advice statement line
   *
   * Notification that the statement line is a debit advice.
   */
  DebitAdviceStatementLine = "56",
  /**
   * Credit advice statement line
   *
   * Notification that the statement line is a credit advice.
   */
  CreditAdviceStatementLine = "57",
  /**
   * Grouped credit advices
   *
   * Notification that the credit advices are grouped.
   */
  GroupedCreditAdvices = "58",
  /**
   * Grouped debit advices
   *
   * Notification that the debit advices are grouped.
   */
  GroupedDebitAdvices = "59",
  /**
   * Registered
   *
   * The name is registered.
   */
  Registered = "60",
  /**
   * Payment denied
   *
   * The payment has been denied.
   */
  PaymentDenied = "61",
  /**
   * Approved as amended
   *
   * Approved with modifications.
   */
  ApprovedAsAmended = "62",
  /**
   * Approved as submitted
   *
   * The request has been approved as submitted.
   */
  ApprovedAsSubmitted = "63",
  /**
   * Cancelled, no activity
   *
   * Cancelled due to the lack of activity.
   */
  CancelledNoActivity = "64",
  /**
   * Under investigation
   *
   * Investigation is being done.
   */
  UnderInvestigation = "65",
  /**
   * Initial claim received
   *
   * Notification that the initial claim was received.
   */
  InitialClaimReceived = "66",
  /**
   * Not in process
   *
   * Not in process.
   */
  NotInProcess = "67",
  /**
   * Rejected, duplicate
   *
   * Rejected because it is a duplicate.
   */
  RejectedDuplicate = "68",
  /**
   * Rejected, resubmit with corrections
   *
   * Rejected but may be resubmitted when corrected.
   */
  RejectedResubmitWithCorrections = "69",
  /**
   * Pending, incomplete
   *
   * Pending because of incomplete information.
   */
  PendingIncomplete = "70",
  /**
   * Under field office investigation
   *
   * Investigation by the field is being done.
   */
  UnderFieldOfficeInvestigation = "71",
  /**
   * Pending, awaiting additional material
   *
   * Pending awaiting receipt of additional material.
   */
  PendingAwaitingAdditionalMaterial = "72",
  /**
   * Pending, awaiting review
   *
   * Pending while awaiting review.
   */
  PendingAwaitingReview = "73",
  /**
   * Reopened
   *
   * Opened again.
   */
  Reopened = "74",
  /**
   * Processed by primary, forwarded to additional payer(s)
   *
   * This request has been processed by the primary payer and sent to additional payer(s).
   */
  ProcessedByPrimaryForwardedToAdditionalPayerS = "75",
  /**
   * Processed by secondary, forwarded to additional payer(s)
   *
   * This request has been processed by the secondary payer and sent to additional payer(s).
   */
  ProcessedBySecondaryForwardedToAdditionalPayerS = "76",
  /**
   * Processed by tertiary, forwarded to additional payer(s)
   *
   * This request has been processed by the tertiary payer and sent to additional payer(s).
   */
  ProcessedByTertiaryForwardedToAdditionalPayerS = "77",
  /**
   * Previous payment decision reversed
   *
   * A previous payment decision has been reversed.
   */
  PreviousPaymentDecisionReversed = "78",
  /**
   * Not our claim, forwarded to another payer(s)
   *
   * A request does not belong to this payer but has been forwarded to another payer(s).
   */
  NotOurClaimForwardedToAnotherPayerS = "79",
  /**
   * Transferred to correct insurance carrier
   *
   * The request has been transferred to the correct insurance carrier for processing.
   */
  TransferredToCorrectInsuranceCarrier = "80",
  /**
   * Not paid, predetermination pricing only
   *
   * Payment has not been made and the enclosed response is predetermination pricing only.
   */
  NotPaidPredeterminationPricingOnly = "81",
  /**
   * Documentation claim
   *
   * The claim is for documentation purposes only, no payment required.
   */
  DocumentationClaim = "82",
  /**
   * Reviewed
   *
   * Assessed.
   */
  Reviewed = "83",
  /**
   * Repriced
   *
   * This price was changed.
   */
  Repriced = "84",
  /**
   * Audited
   *
   * An official examination has occurred.
   */
  Audited = "85",
  /**
   * Conditionally paid
   *
   * Payment has been conditionally made.
   */
  ConditionallyPaid = "86",
  /**
   * On appeal
   *
   * Reconsideration of the decision has been applied for.
   */
  OnAppeal = "87",
  /**
   * Closed
   *
   * Shut.
   */
  Closed = "88",
  /**
   * Reaudited
   *
   * A subsequent official examination has occurred.
   */
  Reaudited = "89",
  /**
   * Reissued
   *
   * Issued again.
   */
  Reissued = "90",
  /**
   * Closed after reopening
   *
   * Reopened and then closed.
   */
  ClosedAfterReopening = "91",
  /**
   * Redetermined
   *
   * Determined again or differently.
   */
  Redetermined = "92",
  /**
   * Processed as primary
   *
   * Processed as the first.
   */
  ProcessedAsPrimary = "93",
  /**
   * Processed as secondary
   *
   * Processed as the second.
   */
  ProcessedAsSecondary = "94",
  /**
   * Processed as tertiary
   *
   * Processed as the third.
   */
  ProcessedAsTertiary = "95",
  /**
   * Correction of error
   *
   * A correction to information previously communicated which contained an error.
   */
  CorrectionError = "96",
  /**
   * Single credit item of a group
   *
   * Notification that the credit item is a single credit item of a group of credit items.
   */
  SingleCreditItemAGroup = "97",
  /**
   * Single debit item of a group
   *
   * Notification that the debit item is a single debit item of a group of debit items.
   */
  SingleDebitItemAGroup = "98",
  /**
   * Interim response
   *
   * The response is an interim one.
   */
  InterimResponse = "99",
  /**
   * Final response
   *
   * The response is an final one.
   */
  FinalResponse = "100",
  /**
   * Debit advice requested
   *
   * A debit advice is requested for the transaction.
   */
  DebitAdviceRequested = "101",
  /**
   * Transaction not impacted
   *
   * Advice that the transaction is not impacted.
   */
  TransactionNotImpacted = "102",
  /**
   * Patient to be notified
   *
   * The action to take is to notify the patient.
   */
  PatientToBeNotified = "103",
  /**
   * Healthcare provider to be notified
   *
   * The action to take is to notify the healthcare provider.
   */
  HealthcareProviderToBeNotified = "104",
  /**
   * Usual general practitioner to be notified
   *
   * The action to take is to notify the usual general practitioner.
   */
  UsualGeneralPractitionerToBeNotified = "105",
  /**
   * Advice without details
   *
   * An advice without details is requested or notified.
   */
  AdviceWithoutDetails = "106",
  /**
   * Advice with details
   *
   * An advice with details is requested or notified.
   */
  AdviceWithDetails = "107",
  /**
   * Amendment requested
   *
   * An amendment is requested.
   */
  AmendmentRequested = "108",
  /**
   * For information
   *
   * Included for information only.
   */
  ForInformation = "109",
  /**
   * Withdraw
   *
   * A code indicating discontinuance or retraction.
   */
  Withdraw = "110",
  /**
   * Delivery date change
   *
   * The action / notiification is a change of the delivery date.
   */
  DeliveryDateChange = "111",
  /**
   * Quantity change
   *
   * The action / notification is a change of quantity.
   */
  QuantityChange = "112",
  /**
   * Resale and claim
   *
   * The identified items have been sold by the distributor to the end customer, and compensation for the loss of inventory value is claimed.
   */
  ResaleAndClaim = "113",
  /**
   * Resale
   *
   * The identified items have been sold by the distributor to the end customer.
   */
  Resale = "114",
  /**
   * Prior addition
   *
   * This existing line item becomes available at an earlier date.
   */
  PriorAddition = "115",
  /**
   * Expired
   *
   * This line has expired.
   */
  Expired = "116",
  /**
   * Hold
   *
   * This line is on Hold.
   */
  Hold = "117",
  /**
   * Open
   *
   * This line is open.
   */
  Open = "118",
  /**
   * Observe
   *
   * The object or item is to be or has been observed.
   */
  Observe = "119",
}

export function description(value: LineStatus): string {
  switch (value) {
    case LineStatus.Added:
      return "Added";
    case LineStatus.Deleted:
      return "Deleted";
    case LineStatus.Changed:
      return "Changed";
    case LineStatus.NoAction:
      return "No action";
    case LineStatus.AcceptedWithoutAmendment:
      return "Accepted without amendment";
    case LineStatus.AcceptedWithAmendment:
      return "Accepted with amendment";
    case LineStatus.NotAccepted:
      return "Not accepted";
    case LineStatus.ScheduleOnly:
      return "Schedule only";
    case LineStatus.Amendments:
      return "Amendments";
    case LineStatus.NotFound:
      return "Not found";
    case LineStatus.NotAmended:
      return "Not amended";
    case LineStatus.LineItemNumbersChanged:
      return "Line item numbers changed";
    case LineStatus.BuyerHasDeductedAmount:
      return "Buyer has deducted amount";
    case LineStatus.BuyerClaimsAgainstInvoice:
      return "Buyer claims against invoice";
    case LineStatus.ChargeBackBySeller:
      return "Charge back by seller";
    case LineStatus.SellerWillIssueCreditNote:
      return "Seller will issue credit note";
    case LineStatus.TermsChangedForNewTerms:
      return "Terms changed for new terms";
    case LineStatus.AbideOutcomeNegotiations:
      return "Abide outcome of negotiations";
    case LineStatus.SellerRejectsDispute:
      return "Seller rejects dispute";
    case LineStatus.Settlement:
      return "Settlement";
    case LineStatus.NoDelivery:
      return "No delivery";
    case LineStatus.CallOffDelivery:
      return "Call-off delivery";
    case LineStatus.ProposedAmendment:
      return "Proposed amendment";
    case LineStatus.AcceptedWithAmendmentNoConfirmationRequired:
      return "Accepted with amendment, no confirmation required";
    case LineStatus.EquipmentProvisionallyRepaired:
      return "Equipment provisionally repaired";
    case LineStatus.Included:
      return "Included";
    case LineStatus.UponReceiptAndVerificationDocumentsWeShallCoverYouWhenDueAsPerYourInstructions:
      return "Upon receipt and verification of documents we shall cover you when due as per your instructions";
    case LineStatus.UponReceiptAndVerificationDocumentsWeShallAuthorizeYouToDebitOurAccountWithYouWhenDue:
      return "Upon receipt and verification of documents we shall authorize you to debit our account with you when due";
    case LineStatus.OnReceiptYourAuthenticatedAdviceWeShallCoverYouWhenDueAsPerYourInstructions:
      return "On receipt of your authenticated advice we shall cover you when due as per your instructions";
    case LineStatus.OnReceiptYourAuthenticatedAdviceWeShallAuthorizeYouToDebitOurAccountWithYouWhenDue:
      return "On receipt of your authenticated advice we shall authorize you to debit our account with you when due";
    case LineStatus.OnReceiptYourAuthenticatedAdviceWeShallCreditYourAccountWithUsWhenDue:
      return "On receipt of your authenticated advice we shall credit your account with us when due";
    case LineStatus.CreditAdviceRequestedForDirectDebit:
      return "Credit advice requested for direct debit";
    case LineStatus.CreditAdviceAndAcknowledgementForDirectDebit:
      return "Credit advice and acknowledgement for direct debit";
    case LineStatus.Inquiry:
      return "Inquiry";
    case LineStatus.Checked:
      return "Checked";
    case LineStatus.NotChecked:
      return "Not checked";
    case LineStatus.Cancelled:
      return "Cancelled";
    case LineStatus.Replaced:
      return "Replaced";
    case LineStatus.New:
      return "New";
    case LineStatus.Agreed:
      return "Agreed";
    case LineStatus.Proposed:
      return "Proposed";
    case LineStatus.AlreadyDelivered:
      return "Already delivered";
    case LineStatus.AdditionalSubordinateStructuresWillFollow:
      return "Additional subordinate structures will follow";
    case LineStatus.AdditionalSubordinateStructuresWillNotFollow:
      return "Additional subordinate structures will not follow";
    case LineStatus.ResultOpposed:
      return "Result opposed";
    case LineStatus.AuctionHeld:
      return "Auction held";
    case LineStatus.LegalActionPursued:
      return "Legal action pursued";
    case LineStatus.MeetingHeld:
      return "Meeting held";
    case LineStatus.ResultSetAside:
      return "Result set aside";
    case LineStatus.ResultDisputed:
      return "Result disputed";
    case LineStatus.Countersued:
      return "Countersued";
    case LineStatus.Pending:
      return "Pending";
    case LineStatus.CourtActionDismissed:
      return "Court action dismissed";
    case LineStatus.ReferredItemAccepted:
      return "Referred item, accepted";
    case LineStatus.ReferredItemRejected:
      return "Referred item, rejected";
    case LineStatus.DebitAdviceStatementLine:
      return "Debit advice statement line";
    case LineStatus.CreditAdviceStatementLine:
      return "Credit advice statement line";
    case LineStatus.GroupedCreditAdvices:
      return "Grouped credit advices";
    case LineStatus.GroupedDebitAdvices:
      return "Grouped debit advices";
    case LineStatus.Registered:
      return "Registered";
    case LineStatus.PaymentDenied:
      return "Payment denied";
    case LineStatus.ApprovedAsAmended:
      return "Approved as amended";
    case LineStatus.ApprovedAsSubmitted:
      return "Approved as submitted";
    case LineStatus.CancelledNoActivity:
      return "Cancelled, no activity";
    case LineStatus.UnderInvestigation:
      return "Under investigation";
    case LineStatus.InitialClaimReceived:
      return "Initial claim received";
    case LineStatus.NotInProcess:
      return "Not in process";
    case LineStatus.RejectedDuplicate:
      return "Rejected, duplicate";
    case LineStatus.RejectedResubmitWithCorrections:
      return "Rejected, resubmit with corrections";
    case LineStatus.PendingIncomplete:
      return "Pending, incomplete";
    case LineStatus.UnderFieldOfficeInvestigation:
      return "Under field office investigation";
    case LineStatus.PendingAwaitingAdditionalMaterial:
      return "Pending, awaiting additional material";
    case LineStatus.PendingAwaitingReview:
      return "Pending, awaiting review";
    case LineStatus.Reopened:
      return "Reopened";
    case LineStatus.ProcessedByPrimaryForwardedToAdditionalPayerS:
      return "Processed by primary, forwarded to additional payer(s)";
    case LineStatus.ProcessedBySecondaryForwardedToAdditionalPayerS:
      return "Processed by secondary, forwarded to additional payer(s)";
    case LineStatus.ProcessedByTertiaryForwardedToAdditionalPayerS:
      return "Processed by tertiary, forwarded to additional payer(s)";
    case LineStatus.PreviousPaymentDecisionReversed:
      return "Previous payment decision reversed";
    case LineStatus.NotOurClaimForwardedToAnotherPayerS:
      return "Not our claim, forwarded to another payer(s)";
    case LineStatus.TransferredToCorrectInsuranceCarrier:
      return "Transferred to correct insurance carrier";
    case LineStatus.NotPaidPredeterminationPricingOnly:
      return "Not paid, predetermination pricing only";
    case LineStatus.DocumentationClaim:
      return "Documentation claim";
    case LineStatus.Reviewed:
      return "Reviewed";
    case LineStatus.Repriced:
      return "Repriced";
    case LineStatus.Audited:
      return "Audited";
    case LineStatus.ConditionallyPaid:
      return "Conditionally paid";
    case LineStatus.OnAppeal:
      return "On appeal";
    case LineStatus.Closed:
      return "Closed";
    case LineStatus.Reaudited:
      return "Reaudited";
    case LineStatus.Reissued:
      return "Reissued";
    case LineStatus.ClosedAfterReopening:
      return "Closed after reopening";
    case LineStatus.Redetermined:
      return "Redetermined";
    case LineStatus.ProcessedAsPrimary:
      return "Processed as primary";
    case LineStatus.ProcessedAsSecondary:
      return "Processed as secondary";
    case LineStatus.ProcessedAsTertiary:
      return "Processed as tertiary";
    case LineStatus.CorrectionError:
      return "Correction of error";
    case LineStatus.SingleCreditItemAGroup:
      return "Single credit item of a group";
    case LineStatus.SingleDebitItemAGroup:
      return "Single debit item of a group";
    case LineStatus.InterimResponse:
      return "Interim response";
    case LineStatus.FinalResponse:
      return "Final response";
    case LineStatus.DebitAdviceRequested:
      return "Debit advice requested";
    case LineStatus.TransactionNotImpacted:
      return "Transaction not impacted";
    case LineStatus.PatientToBeNotified:
      return "Patient to be notified";
    case LineStatus.HealthcareProviderToBeNotified:
      return "Healthcare provider to be notified";
    case LineStatus.UsualGeneralPractitionerToBeNotified:
      return "Usual general practitioner to be notified";
    case LineStatus.AdviceWithoutDetails:
      return "Advice without details";
    case LineStatus.AdviceWithDetails:
      return "Advice with details";
    case LineStatus.AmendmentRequested:
      return "Amendment requested";
    case LineStatus.ForInformation:
      return "For information";
    case LineStatus.Withdraw:
      return "Withdraw";
    case LineStatus.DeliveryDateChange:
      return "Delivery date change";
    case LineStatus.QuantityChange:
      return "Quantity change";
    case LineStatus.ResaleAndClaim:
      return "Resale and claim";
    case LineStatus.Resale:
      return "Resale";
    case LineStatus.PriorAddition:
      return "Prior addition";
    case LineStatus.Expired:
      return "Expired";
    case LineStatus.Hold:
      return "Hold";
    case LineStatus.Open:
      return "Open";
    case LineStatus.Observe:
      return "Observe";
  }
}
