// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/types/types/registry';

import type { CordAuthorityMembershipCall, CordAuthorityMembershipError, CordAuthorityMembershipEvent, CordIdentifierCallTypeOf, CordIdentifierError, CordIdentifierEventEntry, CordIdentifierIdentifierTypeOf, CordIdentifierTimepoint, CordRuntimeMaxNewKeyAgreementKeys, CordRuntimeOriginCaller, CordRuntimeRuntime, CordRuntimeRuntimeFreezeReason, CordRuntimeRuntimeHoldReason, CordRuntimeSessionKeys, FinalityGrandpaEquivocationPrecommit, FinalityGrandpaEquivocationPrevote, FinalityGrandpaPrecommit, FinalityGrandpaPrevote, FrameSupportDispatchDispatchClass, FrameSupportDispatchDispatchInfo, FrameSupportDispatchPays, FrameSupportDispatchPerDispatchClassU32, FrameSupportDispatchPerDispatchClassWeight, FrameSupportDispatchPerDispatchClassWeightsPerClass, FrameSupportDispatchRawOrigin, FrameSupportPreimagesBounded, FrameSupportTokensMiscBalanceStatus, FrameSystemAccountInfo, FrameSystemCall, FrameSystemCodeUpgradeAuthorization, FrameSystemError, FrameSystemEvent, FrameSystemEventRecord, FrameSystemExtensionsCheckGenesis, FrameSystemExtensionsCheckNonZeroSender, FrameSystemExtensionsCheckNonce, FrameSystemExtensionsCheckSpecVersion, FrameSystemExtensionsCheckTxVersion, FrameSystemExtensionsCheckWeight, FrameSystemLastRuntimeUpgradeInfo, FrameSystemLimitsBlockLength, FrameSystemLimitsBlockWeights, FrameSystemLimitsWeightsPerClass, FrameSystemPhase, PalletAssetAssetDistributionEntry, PalletAssetAssetEntry, PalletAssetAssetInputEntry, PalletAssetAssetIssuanceEntry, PalletAssetAssetStatusOf, PalletAssetAssetTransferEntry, PalletAssetAssetTypeOf, PalletAssetCall, PalletAssetError, PalletAssetEvent, PalletAssetVcAssetDistributionEntry, PalletAssetVcAssetEntry, PalletBabeCall, PalletBabeError, PalletBalancesAccountData, PalletBalancesAdjustmentDirection, PalletBalancesBalanceLock, PalletBalancesCall, PalletBalancesError, PalletBalancesEvent, PalletBalancesIdAmountRuntimeFreezeReason, PalletBalancesIdAmountRuntimeHoldReason, PalletBalancesReasons, PalletBalancesReserveData, PalletChainSpaceCall, PalletChainSpaceError, PalletChainSpaceEvent, PalletChainSpacePermissions, PalletChainSpaceSpaceAuthorization, PalletChainSpaceSpaceDetails, PalletCollectiveCall, PalletCollectiveError, PalletCollectiveEvent, PalletCollectiveRawOrigin, PalletCollectiveVotes, PalletDidCall, PalletDidDidDetails, PalletDidDidDetailsDidAuthorizedCallOperation, PalletDidDidDetailsDidCreationDetails, PalletDidDidDetailsDidEncryptionKey, PalletDidDidDetailsDidPublicKey, PalletDidDidDetailsDidPublicKeyDetails, PalletDidDidDetailsDidSignature, PalletDidDidDetailsDidVerificationKey, PalletDidError, PalletDidEvent, PalletDidNameCall, PalletDidNameDidNameDidNameOwnership, PalletDidNameError, PalletDidNameEvent, PalletDidOriginDidRawOrigin, PalletDidServiceEndpointsDidEndpoint, PalletGrandpaCall, PalletGrandpaError, PalletGrandpaEvent, PalletGrandpaStoredPendingChange, PalletGrandpaStoredState, PalletIdentityAuthorityProperties, PalletIdentityCall, PalletIdentityError, PalletIdentityEvent, PalletIdentityJudgement, PalletIdentityLegacyIdentityInfo, PalletIdentityRegistrarInfo, PalletIdentityRegistration, PalletImOnlineCall, PalletImOnlineError, PalletImOnlineEvent, PalletImOnlineHeartbeat, PalletImOnlineSr25519AppSr25519Public, PalletImOnlineSr25519AppSr25519Signature, PalletIndicesCall, PalletIndicesError, PalletIndicesEvent, PalletMembershipCall, PalletMembershipError, PalletMembershipEvent, PalletMultisigCall, PalletMultisigError, PalletMultisigEvent, PalletMultisigMultisig, PalletMultisigTimepoint, PalletNetworkMembershipCall, PalletNetworkMembershipCheckNetworkMembership, PalletNetworkMembershipError, PalletNetworkMembershipEvent, PalletNetworkMembershipMemberData, PalletNetworkScoreAggregatedEntryOf, PalletNetworkScoreCall, PalletNetworkScoreEntryTypeOf, PalletNetworkScoreError, PalletNetworkScoreEvent, PalletNetworkScoreRatingEntry, PalletNetworkScoreRatingInputEntry, PalletNetworkScoreRatingTypeOf, PalletNodeAuthorizationCall, PalletNodeAuthorizationError, PalletNodeAuthorizationEvent, PalletNodeAuthorizationNodeInfo, PalletOffencesEvent, PalletPreimageCall, PalletPreimageError, PalletPreimageEvent, PalletPreimageHoldReason, PalletPreimageOldRequestStatus, PalletPreimageRequestStatus, PalletRemarkCall, PalletRemarkError, PalletRemarkEvent, PalletRuntimeUpgradeCall, PalletSchedulerCall, PalletSchedulerError, PalletSchedulerEvent, PalletSchedulerRetryConfig, PalletSchedulerScheduled, PalletSchemaCall, PalletSchemaError, PalletSchemaEvent, PalletSchemaSchemaEntry, PalletSessionCall, PalletSessionError, PalletSessionEvent, PalletStatementCall, PalletStatementError, PalletStatementEvent, PalletStatementPresentationTypeOf, PalletStatementStatementDetails, PalletStatementStatementEntryStatus, PalletStatementStatementPresentationDetails, PalletSudoCall, PalletSudoError, PalletSudoEvent, PalletTimestampCall, PalletUtilityCall, PalletUtilityError, PalletUtilityEvent, SpArithmeticArithmeticError, SpAuthorityDiscoveryAppPublic, SpConsensusBabeAllowedSlots, SpConsensusBabeAppPublic, SpConsensusBabeBabeEpochConfiguration, SpConsensusBabeDigestsNextConfigDescriptor, SpConsensusBabeDigestsPreDigest, SpConsensusBabeDigestsPrimaryPreDigest, SpConsensusBabeDigestsSecondaryPlainPreDigest, SpConsensusBabeDigestsSecondaryVRFPreDigest, SpConsensusGrandpaAppPublic, SpConsensusGrandpaAppSignature, SpConsensusGrandpaEquivocation, SpConsensusGrandpaEquivocationProof, SpConsensusSlotsEquivocationProof, SpCoreCryptoKeyTypeId, SpCoreEcdsaPublic, SpCoreEcdsaSignature, SpCoreEd25519Public, SpCoreEd25519Signature, SpCoreSr25519Public, SpCoreSr25519Signature, SpCoreSr25519VrfVrfSignature, SpCoreVoid, SpRuntimeBlakeTwo256, SpRuntimeDigest, SpRuntimeDigestDigestItem, SpRuntimeDispatchError, SpRuntimeHeader, SpRuntimeModuleError, SpRuntimeMultiSignature, SpRuntimeTokenError, SpRuntimeTransactionalError, SpSessionMembershipProof, SpStakingOffenceOffenceDetails, SpVersionRuntimeVersion, SpWeightsRuntimeDbWeight, SpWeightsWeightV2Weight } from '@polkadot/types/lookup';

declare module '@polkadot/types/types/registry' {
  interface InterfaceTypes {
    CordAuthorityMembershipCall: CordAuthorityMembershipCall;
    CordAuthorityMembershipError: CordAuthorityMembershipError;
    CordAuthorityMembershipEvent: CordAuthorityMembershipEvent;
    CordIdentifierCallTypeOf: CordIdentifierCallTypeOf;
    CordIdentifierError: CordIdentifierError;
    CordIdentifierEventEntry: CordIdentifierEventEntry;
    CordIdentifierIdentifierTypeOf: CordIdentifierIdentifierTypeOf;
    CordIdentifierTimepoint: CordIdentifierTimepoint;
    CordRuntimeMaxNewKeyAgreementKeys: CordRuntimeMaxNewKeyAgreementKeys;
    CordRuntimeOriginCaller: CordRuntimeOriginCaller;
    CordRuntimeRuntime: CordRuntimeRuntime;
    CordRuntimeRuntimeFreezeReason: CordRuntimeRuntimeFreezeReason;
    CordRuntimeRuntimeHoldReason: CordRuntimeRuntimeHoldReason;
    CordRuntimeSessionKeys: CordRuntimeSessionKeys;
    FinalityGrandpaEquivocationPrecommit: FinalityGrandpaEquivocationPrecommit;
    FinalityGrandpaEquivocationPrevote: FinalityGrandpaEquivocationPrevote;
    FinalityGrandpaPrecommit: FinalityGrandpaPrecommit;
    FinalityGrandpaPrevote: FinalityGrandpaPrevote;
    FrameSupportDispatchDispatchClass: FrameSupportDispatchDispatchClass;
    FrameSupportDispatchDispatchInfo: FrameSupportDispatchDispatchInfo;
    FrameSupportDispatchPays: FrameSupportDispatchPays;
    FrameSupportDispatchPerDispatchClassU32: FrameSupportDispatchPerDispatchClassU32;
    FrameSupportDispatchPerDispatchClassWeight: FrameSupportDispatchPerDispatchClassWeight;
    FrameSupportDispatchPerDispatchClassWeightsPerClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
    FrameSupportDispatchRawOrigin: FrameSupportDispatchRawOrigin;
    FrameSupportPreimagesBounded: FrameSupportPreimagesBounded;
    FrameSupportTokensMiscBalanceStatus: FrameSupportTokensMiscBalanceStatus;
    FrameSystemAccountInfo: FrameSystemAccountInfo;
    FrameSystemCall: FrameSystemCall;
    FrameSystemCodeUpgradeAuthorization: FrameSystemCodeUpgradeAuthorization;
    FrameSystemError: FrameSystemError;
    FrameSystemEvent: FrameSystemEvent;
    FrameSystemEventRecord: FrameSystemEventRecord;
    FrameSystemExtensionsCheckGenesis: FrameSystemExtensionsCheckGenesis;
    FrameSystemExtensionsCheckNonZeroSender: FrameSystemExtensionsCheckNonZeroSender;
    FrameSystemExtensionsCheckNonce: FrameSystemExtensionsCheckNonce;
    FrameSystemExtensionsCheckSpecVersion: FrameSystemExtensionsCheckSpecVersion;
    FrameSystemExtensionsCheckTxVersion: FrameSystemExtensionsCheckTxVersion;
    FrameSystemExtensionsCheckWeight: FrameSystemExtensionsCheckWeight;
    FrameSystemLastRuntimeUpgradeInfo: FrameSystemLastRuntimeUpgradeInfo;
    FrameSystemLimitsBlockLength: FrameSystemLimitsBlockLength;
    FrameSystemLimitsBlockWeights: FrameSystemLimitsBlockWeights;
    FrameSystemLimitsWeightsPerClass: FrameSystemLimitsWeightsPerClass;
    FrameSystemPhase: FrameSystemPhase;
    PalletAssetAssetDistributionEntry: PalletAssetAssetDistributionEntry;
    PalletAssetAssetEntry: PalletAssetAssetEntry;
    PalletAssetAssetInputEntry: PalletAssetAssetInputEntry;
    PalletAssetAssetIssuanceEntry: PalletAssetAssetIssuanceEntry;
    PalletAssetAssetStatusOf: PalletAssetAssetStatusOf;
    PalletAssetAssetTransferEntry: PalletAssetAssetTransferEntry;
    PalletAssetAssetTypeOf: PalletAssetAssetTypeOf;
    PalletAssetCall: PalletAssetCall;
    PalletAssetError: PalletAssetError;
    PalletAssetEvent: PalletAssetEvent;
    PalletAssetVcAssetDistributionEntry: PalletAssetVcAssetDistributionEntry;
    PalletAssetVcAssetEntry: PalletAssetVcAssetEntry;
    PalletBabeCall: PalletBabeCall;
    PalletBabeError: PalletBabeError;
    PalletBalancesAccountData: PalletBalancesAccountData;
    PalletBalancesAdjustmentDirection: PalletBalancesAdjustmentDirection;
    PalletBalancesBalanceLock: PalletBalancesBalanceLock;
    PalletBalancesCall: PalletBalancesCall;
    PalletBalancesError: PalletBalancesError;
    PalletBalancesEvent: PalletBalancesEvent;
    PalletBalancesIdAmountRuntimeFreezeReason: PalletBalancesIdAmountRuntimeFreezeReason;
    PalletBalancesIdAmountRuntimeHoldReason: PalletBalancesIdAmountRuntimeHoldReason;
    PalletBalancesReasons: PalletBalancesReasons;
    PalletBalancesReserveData: PalletBalancesReserveData;
    PalletChainSpaceCall: PalletChainSpaceCall;
    PalletChainSpaceError: PalletChainSpaceError;
    PalletChainSpaceEvent: PalletChainSpaceEvent;
    PalletChainSpacePermissions: PalletChainSpacePermissions;
    PalletChainSpaceSpaceAuthorization: PalletChainSpaceSpaceAuthorization;
    PalletChainSpaceSpaceDetails: PalletChainSpaceSpaceDetails;
    PalletCollectiveCall: PalletCollectiveCall;
    PalletCollectiveError: PalletCollectiveError;
    PalletCollectiveEvent: PalletCollectiveEvent;
    PalletCollectiveRawOrigin: PalletCollectiveRawOrigin;
    PalletCollectiveVotes: PalletCollectiveVotes;
    PalletDidCall: PalletDidCall;
    PalletDidDidDetails: PalletDidDidDetails;
    PalletDidDidDetailsDidAuthorizedCallOperation: PalletDidDidDetailsDidAuthorizedCallOperation;
    PalletDidDidDetailsDidCreationDetails: PalletDidDidDetailsDidCreationDetails;
    PalletDidDidDetailsDidEncryptionKey: PalletDidDidDetailsDidEncryptionKey;
    PalletDidDidDetailsDidPublicKey: PalletDidDidDetailsDidPublicKey;
    PalletDidDidDetailsDidPublicKeyDetails: PalletDidDidDetailsDidPublicKeyDetails;
    PalletDidDidDetailsDidSignature: PalletDidDidDetailsDidSignature;
    PalletDidDidDetailsDidVerificationKey: PalletDidDidDetailsDidVerificationKey;
    PalletDidError: PalletDidError;
    PalletDidEvent: PalletDidEvent;
    PalletDidNameCall: PalletDidNameCall;
    PalletDidNameDidNameDidNameOwnership: PalletDidNameDidNameDidNameOwnership;
    PalletDidNameError: PalletDidNameError;
    PalletDidNameEvent: PalletDidNameEvent;
    PalletDidOriginDidRawOrigin: PalletDidOriginDidRawOrigin;
    PalletDidServiceEndpointsDidEndpoint: PalletDidServiceEndpointsDidEndpoint;
    PalletGrandpaCall: PalletGrandpaCall;
    PalletGrandpaError: PalletGrandpaError;
    PalletGrandpaEvent: PalletGrandpaEvent;
    PalletGrandpaStoredPendingChange: PalletGrandpaStoredPendingChange;
    PalletGrandpaStoredState: PalletGrandpaStoredState;
    PalletIdentityAuthorityProperties: PalletIdentityAuthorityProperties;
    PalletIdentityCall: PalletIdentityCall;
    PalletIdentityError: PalletIdentityError;
    PalletIdentityEvent: PalletIdentityEvent;
    PalletIdentityJudgement: PalletIdentityJudgement;
    PalletIdentityLegacyIdentityInfo: PalletIdentityLegacyIdentityInfo;
    PalletIdentityRegistrarInfo: PalletIdentityRegistrarInfo;
    PalletIdentityRegistration: PalletIdentityRegistration;
    PalletImOnlineCall: PalletImOnlineCall;
    PalletImOnlineError: PalletImOnlineError;
    PalletImOnlineEvent: PalletImOnlineEvent;
    PalletImOnlineHeartbeat: PalletImOnlineHeartbeat;
    PalletImOnlineSr25519AppSr25519Public: PalletImOnlineSr25519AppSr25519Public;
    PalletImOnlineSr25519AppSr25519Signature: PalletImOnlineSr25519AppSr25519Signature;
    PalletIndicesCall: PalletIndicesCall;
    PalletIndicesError: PalletIndicesError;
    PalletIndicesEvent: PalletIndicesEvent;
    PalletMembershipCall: PalletMembershipCall;
    PalletMembershipError: PalletMembershipError;
    PalletMembershipEvent: PalletMembershipEvent;
    PalletMultisigCall: PalletMultisigCall;
    PalletMultisigError: PalletMultisigError;
    PalletMultisigEvent: PalletMultisigEvent;
    PalletMultisigMultisig: PalletMultisigMultisig;
    PalletMultisigTimepoint: PalletMultisigTimepoint;
    PalletNetworkMembershipCall: PalletNetworkMembershipCall;
    PalletNetworkMembershipCheckNetworkMembership: PalletNetworkMembershipCheckNetworkMembership;
    PalletNetworkMembershipError: PalletNetworkMembershipError;
    PalletNetworkMembershipEvent: PalletNetworkMembershipEvent;
    PalletNetworkMembershipMemberData: PalletNetworkMembershipMemberData;
    PalletNetworkScoreAggregatedEntryOf: PalletNetworkScoreAggregatedEntryOf;
    PalletNetworkScoreCall: PalletNetworkScoreCall;
    PalletNetworkScoreEntryTypeOf: PalletNetworkScoreEntryTypeOf;
    PalletNetworkScoreError: PalletNetworkScoreError;
    PalletNetworkScoreEvent: PalletNetworkScoreEvent;
    PalletNetworkScoreRatingEntry: PalletNetworkScoreRatingEntry;
    PalletNetworkScoreRatingInputEntry: PalletNetworkScoreRatingInputEntry;
    PalletNetworkScoreRatingTypeOf: PalletNetworkScoreRatingTypeOf;
    PalletNodeAuthorizationCall: PalletNodeAuthorizationCall;
    PalletNodeAuthorizationError: PalletNodeAuthorizationError;
    PalletNodeAuthorizationEvent: PalletNodeAuthorizationEvent;
    PalletNodeAuthorizationNodeInfo: PalletNodeAuthorizationNodeInfo;
    PalletOffencesEvent: PalletOffencesEvent;
    PalletPreimageCall: PalletPreimageCall;
    PalletPreimageError: PalletPreimageError;
    PalletPreimageEvent: PalletPreimageEvent;
    PalletPreimageHoldReason: PalletPreimageHoldReason;
    PalletPreimageOldRequestStatus: PalletPreimageOldRequestStatus;
    PalletPreimageRequestStatus: PalletPreimageRequestStatus;
    PalletRemarkCall: PalletRemarkCall;
    PalletRemarkError: PalletRemarkError;
    PalletRemarkEvent: PalletRemarkEvent;
    PalletRuntimeUpgradeCall: PalletRuntimeUpgradeCall;
    PalletSchedulerCall: PalletSchedulerCall;
    PalletSchedulerError: PalletSchedulerError;
    PalletSchedulerEvent: PalletSchedulerEvent;
    PalletSchedulerRetryConfig: PalletSchedulerRetryConfig;
    PalletSchedulerScheduled: PalletSchedulerScheduled;
    PalletSchemaCall: PalletSchemaCall;
    PalletSchemaError: PalletSchemaError;
    PalletSchemaEvent: PalletSchemaEvent;
    PalletSchemaSchemaEntry: PalletSchemaSchemaEntry;
    PalletSessionCall: PalletSessionCall;
    PalletSessionError: PalletSessionError;
    PalletSessionEvent: PalletSessionEvent;
    PalletStatementCall: PalletStatementCall;
    PalletStatementError: PalletStatementError;
    PalletStatementEvent: PalletStatementEvent;
    PalletStatementPresentationTypeOf: PalletStatementPresentationTypeOf;
    PalletStatementStatementDetails: PalletStatementStatementDetails;
    PalletStatementStatementEntryStatus: PalletStatementStatementEntryStatus;
    PalletStatementStatementPresentationDetails: PalletStatementStatementPresentationDetails;
    PalletSudoCall: PalletSudoCall;
    PalletSudoError: PalletSudoError;
    PalletSudoEvent: PalletSudoEvent;
    PalletTimestampCall: PalletTimestampCall;
    PalletUtilityCall: PalletUtilityCall;
    PalletUtilityError: PalletUtilityError;
    PalletUtilityEvent: PalletUtilityEvent;
    SpArithmeticArithmeticError: SpArithmeticArithmeticError;
    SpAuthorityDiscoveryAppPublic: SpAuthorityDiscoveryAppPublic;
    SpConsensusBabeAllowedSlots: SpConsensusBabeAllowedSlots;
    SpConsensusBabeAppPublic: SpConsensusBabeAppPublic;
    SpConsensusBabeBabeEpochConfiguration: SpConsensusBabeBabeEpochConfiguration;
    SpConsensusBabeDigestsNextConfigDescriptor: SpConsensusBabeDigestsNextConfigDescriptor;
    SpConsensusBabeDigestsPreDigest: SpConsensusBabeDigestsPreDigest;
    SpConsensusBabeDigestsPrimaryPreDigest: SpConsensusBabeDigestsPrimaryPreDigest;
    SpConsensusBabeDigestsSecondaryPlainPreDigest: SpConsensusBabeDigestsSecondaryPlainPreDigest;
    SpConsensusBabeDigestsSecondaryVRFPreDigest: SpConsensusBabeDigestsSecondaryVRFPreDigest;
    SpConsensusGrandpaAppPublic: SpConsensusGrandpaAppPublic;
    SpConsensusGrandpaAppSignature: SpConsensusGrandpaAppSignature;
    SpConsensusGrandpaEquivocation: SpConsensusGrandpaEquivocation;
    SpConsensusGrandpaEquivocationProof: SpConsensusGrandpaEquivocationProof;
    SpConsensusSlotsEquivocationProof: SpConsensusSlotsEquivocationProof;
    SpCoreCryptoKeyTypeId: SpCoreCryptoKeyTypeId;
    SpCoreEcdsaPublic: SpCoreEcdsaPublic;
    SpCoreEcdsaSignature: SpCoreEcdsaSignature;
    SpCoreEd25519Public: SpCoreEd25519Public;
    SpCoreEd25519Signature: SpCoreEd25519Signature;
    SpCoreSr25519Public: SpCoreSr25519Public;
    SpCoreSr25519Signature: SpCoreSr25519Signature;
    SpCoreSr25519VrfVrfSignature: SpCoreSr25519VrfVrfSignature;
    SpCoreVoid: SpCoreVoid;
    SpRuntimeBlakeTwo256: SpRuntimeBlakeTwo256;
    SpRuntimeDigest: SpRuntimeDigest;
    SpRuntimeDigestDigestItem: SpRuntimeDigestDigestItem;
    SpRuntimeDispatchError: SpRuntimeDispatchError;
    SpRuntimeHeader: SpRuntimeHeader;
    SpRuntimeModuleError: SpRuntimeModuleError;
    SpRuntimeMultiSignature: SpRuntimeMultiSignature;
    SpRuntimeTokenError: SpRuntimeTokenError;
    SpRuntimeTransactionalError: SpRuntimeTransactionalError;
    SpSessionMembershipProof: SpSessionMembershipProof;
    SpStakingOffenceOffenceDetails: SpStakingOffenceOffenceDetails;
    SpVersionRuntimeVersion: SpVersionRuntimeVersion;
    SpWeightsRuntimeDbWeight: SpWeightsRuntimeDbWeight;
    SpWeightsWeightV2Weight: SpWeightsWeightV2Weight;
  } // InterfaceTypes
} // declare module