//! Error types

use pinocchio::program_error::ProgramError;

/// Errors that may be returned by the Metadata program.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MetadataError {
    /// 0 Failed to unpack instruction data
    InstructionUnpackError,

    /// Failed to pack instruction data
    InstructionPackError,

    /// Lamport balance below rent-exempt threshold.
    NotRentExempt,

    /// Already initialized
    AlreadyInitialized,

    /// Uninitialized
    Uninitialized,

    ///  Metadata's key must match seed of ['metadata', program id, mint] provided
    InvalidMetadataKey,

    ///  Edition's key must match seed of ['metadata', program id, name, 'edition'] provided
    InvalidEditionKey,

    /// Update Authority given does not match
    UpdateAuthorityIncorrect,

    /// Update Authority needs to be signer to update metadata
    UpdateAuthorityIsNotSigner,

    /// You must be the mint authority and signer on this transaction
    NotMintAuthority,

    /// 10 - Mint authority provided does not match the authority on the mint
    InvalidMintAuthority,

    /// Name too long
    NameTooLong,

    /// Symbol too long
    SymbolTooLong,

    /// URI too long
    UriTooLong,

    /// Update authority must be equivalent to the metadata's authority and also signer of this transaction
    UpdateAuthorityMustBeEqualToMetadataAuthorityAndSigner,

    /// Mint given does not match mint on Metadata
    MintMismatch,

    /// Editions must have exactly one token
    EditionsMustHaveExactlyOneToken,

    /// Maximum editions printed already
    MaxEditionsMintedAlready,

    /// Token mint to failed
    TokenMintToFailed,

    /// The master edition record passed must match the master record on the edition given
    MasterRecordMismatch,

    /// 20 - The destination account does not have the right mint
    DestinationMintMismatch,

    /// An edition can only mint one of its kind!
    EditionAlreadyMinted,

    /// Printing mint decimals should be zero
    PrintingMintDecimalsShouldBeZero,

    /// OneTimePrintingAuthorizationMint mint decimals should be zero
    OneTimePrintingAuthorizationMintDecimalsShouldBeZero,

    /// Edition mint decimals should be zero
    EditionMintDecimalsShouldBeZero,

    /// Token burn failed
    TokenBurnFailed,

    /// The One Time authorization mint does not match that on the token account!
    TokenAccountOneTimeAuthMintMismatch,

    /// Derived key invalid
    DerivedKeyInvalid,

    /// The Printing mint does not match that on the master edition!
    PrintingMintMismatch,

    /// The  One Time Printing Auth mint does not match that on the master edition!
    OneTimePrintingAuthMintMismatch,

    /// 30 - The mint of the token account does not match the Printing mint!
    TokenAccountMintMismatch,

    /// The mint of the token account does not match the master metadata mint!
    TokenAccountMintMismatchV2,

    /// Not enough tokens to mint a limited edition
    NotEnoughTokens,

    /// The mint on your authorization token holding account does not match your Printing mint!
    PrintingMintAuthorizationAccountMismatch,

    /// The authorization token account has a different owner than the update authority for the master edition!
    AuthorizationTokenAccountOwnerMismatch,

    /// This feature is currently disabled.
    Disabled,

    /// Creators list too long
    CreatorsTooLong,

    /// Creators must be at least one if set
    CreatorsMustBeAtleastOne,

    /// If using a creators array, you must be one of the creators listed
    MustBeOneOfCreators,

    /// This metadata does not have creators
    NoCreatorsPresentOnMetadata,

    /// 40 - This creator address was not found
    CreatorNotFound,

    /// Basis points cannot be more than 10000
    InvalidBasisPoints,

    /// Primary sale can only be flipped to true and is immutable
    PrimarySaleCanOnlyBeFlippedToTrue,

    /// Owner does not match that on the account given
    OwnerMismatch,

    /// This account has no tokens to be used for authorization
    NoBalanceInAccountForAuthorization,

    /// Share total must equal 100 for creator array
    ShareTotalMustBe100,

    /// This reservation list already exists!
    ReservationExists,

    /// This reservation list does not exist!
    ReservationDoesNotExist,

    /// This reservation list exists but was never set with reservations
    ReservationNotSet,

    /// This reservation list has already been set!
    ReservationAlreadyMade,

    /// 50 - Provided more addresses than max allowed in single reservation
    BeyondMaxAddressSize,

    /// NumericalOverflowError
    NumericalOverflowError,

    /// This reservation would go beyond the maximum supply of the master edition!
    ReservationBreachesMaximumSupply,

    /// Address not in reservation!
    AddressNotInReservation,

    /// You cannot unilaterally verify another creator, they must sign
    CannotVerifyAnotherCreator,

    /// You cannot unilaterally unverify another creator
    CannotUnverifyAnotherCreator,

    /// In initial reservation setting, spots remaining should equal total spots
    SpotMismatch,

    /// Incorrect account owner
    IncorrectOwner,

    /// printing these tokens would breach the maximum supply limit of the master edition
    PrintingWouldBreachMaximumSupply,

    /// Data is immutable
    DataIsImmutable,

    /// 60 - No duplicate creator addresses
    DuplicateCreatorAddress,

    /// Reservation spots remaining should match total spots when first being created
    ReservationSpotsRemainingShouldMatchTotalSpotsAtStart,

    /// Invalid token program
    InvalidTokenProgram,

    /// Data type mismatch
    DataTypeMismatch,

    /// Beyond alotted address size in reservation!
    BeyondAlottedAddressSize,

    /// The reservation has only been partially alotted
    ReservationNotComplete,

    /// You cannot splice over an existing reservation!
    TriedToReplaceAnExistingReservation,

    /// Invalid operation
    InvalidOperation,

    /// Invalid owner
    InvalidOwner,

    /// Printing mint supply must be zero for conversion
    PrintingMintSupplyMustBeZeroForConversion,

    /// 70 - One Time Auth mint supply must be zero for conversion
    OneTimeAuthMintSupplyMustBeZeroForConversion,

    /// You tried to insert one edition too many into an edition mark pda
    InvalidEditionIndex,

    // In the legacy system the reservation needs to be of size one for cpu limit reasons
    ReservationArrayShouldBeSizeOne,

    /// Is Mutable can only be flipped to false
    IsMutableCanOnlyBeFlippedToFalse,

    CollectionCannotBeVerifiedInThisInstruction,

    Removed, //For the curious we cannot get rid of an instruction in the enum or move them or it will break our api, this is a friendly way to get rid of them

    MustBeBurned,

    InvalidUseMethod,

    CannotChangeUseMethodAfterFirstUse,

    CannotChangeUsesAfterFirstUse,

    // 80
    CollectionNotFound,

    InvalidCollectionUpdateAuthority,

    CollectionMustBeAUniqueMasterEdition,

    UseAuthorityRecordAlreadyExists,

    UseAuthorityRecordAlreadyRevoked,

    Unusable,

    NotEnoughUses,

    InvalidUseAuthorityRecord,

    InvalidCollectionAuthorityRecord,

    InvalidFreezeAuthority,

    InvalidDelegate,

    CannotAdjustVerifiedCreator,

    CannotRemoveVerifiedCreator,

    CannotWipeVerifiedCreators,

    NotAllowedToChangeSellerFeeBasisPoints,

    /// Edition override cannot be zero
    EditionOverrideCannotBeZero,

    InvalidUser,

    /// Revoke Collection Authority signer is incorrect
    RevokeCollectionAuthoritySignerIncorrect,

    // 100
    TokenCloseFailed,

    /// 101 - Calling v1.3 function on unsized collection
    UnsizedCollection,

    /// 102 - Calling v1.2 function on a sized collection
    SizedCollection,

    /// 103 - Missing collection metadata account.
    MissingCollectionMetadata,

    /// 104 - This NFT is not a member of the specified collection.
    NotAMemberOfCollection,

    /// 105 - This NFT is not a verified member of the specified collection.
    NotVerifiedMemberOfCollection,

    /// 106 - This NFT is not a collection parent NFT.
    NotACollectionParent,

    /// 107 - Could not determine a TokenStandard type.
    CouldNotDetermineTokenStandard,

    /// 108 - Missing edition account for a non-fungible token type.
    MissingEditionAccount,

    /// 109 - Not a Master Edition
    NotAMasterEdition,

    /// 110 - Master Edition has prints.
    MasterEditionHasPrints,

    /// 111 - Borsh Deserialization Error
    BorshDeserializationError,

    /// 112 - Cannot update a verified colleciton in this command
    CannotUpdateVerifiedCollection,

    /// 113 - Edition Account Doesnt Match Collection
    CollectionMasterEditionAccountInvalid,

    /// 114 - Item is already verified.
    AlreadyVerified,

    /// 115 - Item is already unverified.
    AlreadyUnverified,

    /// 116 - Not a Print Edition
    NotAPrintEdition,

    /// 117 - Invalid Edition Marker
    InvalidMasterEdition,

    /// 118 - Invalid Edition Marker
    InvalidPrintEdition,

    /// 119 - Invalid Edition Marker
    InvalidEditionMarker,

    /// 120 - Reservation List is Deprecated
    ReservationListDeprecated,

    /// 121 - Print Edition doesn't match Master Edition
    PrintEditionDoesNotMatchMasterEdition,

    /// 122 - Edition Number greater than max supply
    EditionNumberGreaterThanMaxSupply,

    /// 123 - Must unverify before migrating collections.
    MustUnverify,

    /// 124 - Invalid Escrow Account Bump Seed
    InvalidEscrowBumpSeed,

    /// 125 - Must be Escrow Authority
    MustBeEscrowAuthority,

    /// 126 - Invalid System Program
    InvalidSystemProgram,

    /// 127 - Must be a Non Fungible Token
    MustBeNonFungible,

    /// 128 - Insufficient tokens for transfer
    InsufficientTokens,

    /// 129 - Borsh Serialization Error
    BorshSerializationError,

    /// 130 - Cannot create NFT with no Freeze Authority.
    NoFreezeAuthoritySet,

    /// 131
    InvalidCollectionSizeChange,

    /// 132
    InvalidBubblegumSigner,

    /// 133
    EscrowParentHasDelegate,

    /// 134
    MintIsNotSigner,

    /// 135
    InvalidTokenStandard,

    /// 136
    InvalidMintForTokenStandard,

    /// 137
    InvalidAuthorizationRules,

    /// 138
    MissingAuthorizationRules,

    /// 139
    MissingProgrammableConfig,

    /// 140
    InvalidProgrammableConfig,

    /// 141
    DelegateAlreadyExists,

    /// 142
    DelegateNotFound,

    /// 143
    MissingAccountInBuilder,

    /// 144
    MissingArgumentInBuilder,

    /// 145
    FeatureNotSupported,

    /// 146
    InvalidSystemWallet,

    /// 147
    OnlySaleDelegateCanTransfer,

    /// 148
    MissingTokenAccount,

    /// 149
    MissingSplTokenProgram,

    /// 150
    MissingAuthorizationRulesProgram,

    /// 151
    InvalidDelegateRoleForTransfer,

    /// 152
    InvalidTransferAuthority,

    /// 153
    InstructionNotSupported,

    /// 154
    KeyMismatch,

    /// 155
    LockedToken,

    /// 156
    UnlockedToken,

    /// 157
    MissingDelegateRole,

    /// 158
    InvalidAuthorityType,

    /// 159
    MissingTokenRecord,

    /// 160
    MintSupplyMustBeZero,

    /// 161
    DataIsEmptyOrZeroed,

    /// 162
    MissingTokenOwnerAccount,

    /// 163
    InvalidMasterEditionAccountLength,

    /// 164
    IncorrectTokenState,

    /// 165
    InvalidDelegateRole,

    /// 166
    MissingPrintSupply,

    /// 167
    MissingMasterEditionAccount,

    /// 168
    AmountMustBeGreaterThanZero,

    /// 169
    InvalidDelegateArgs,

    /// 170
    MissingLockedTransferAddress,

    /// 171
    InvalidLockedTransferAddress,

    /// 172
    DataIncrementLimitExceeded,

    /// 173
    CannotUpdateAssetWithDelegate,

    /// 174
    InvalidAmount,

    /// 175
    MissingMasterEditionMintAccount,

    /// 176
    MissingMasterEditionTokenAccount,

    /// 177
    MissingEditionMarkerAccount,

    /// 178
    CannotBurnWithDelegate,

    /// 179
    MissingEdition,

    /// 180
    InvalidAssociatedTokenAccountProgram,

    /// 181
    InvalidInstructionsSysvar,

    /// 182
    InvalidParentAccounts,

    /// 183
    InvalidUpdateArgs,

    /// 184
    InsufficientTokenBalance,

    /// 185
    MissingCollectionMint,

    /// 186
    MissingCollectionMasterEdition,

    /// 187
    InvalidTokenRecord,

    /// 188
    InvalidCloseAuthority,

    /// 189
    InvalidInstruction,

    /// 190
    MissingDelegateRecord,

    /// 191
    InvalidFeeAccount,

    /// 192
    InvalidMetadataFlags,

    /// 193
    CannotChangeUpdateAuthorityWithDelegate,

    /// 194
    InvalidMintExtensionType,

    /// 195
    InvalidMintCloseAuthority,

    /// 196
    InvalidMetadataPointer,

    /// 197
    InvalidTokenExtensionType,

    /// 198
    MissingImmutableOwnerExtension,

    /// 199
    ExpectedUninitializedAccount,

    /// 200
    InvalidEditionAccountLength,

    /// 201
    AccountAlreadyResized,

    /// 202
    ConditionsForClosingNotMet,
}

impl From<MetadataError> for ProgramError {
    fn from(e: MetadataError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
