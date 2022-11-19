#[doc = "The MAC command identifier"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct R {
    bits: u8,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct W {
    bits: u8,
}
impl core::ops::Deref for W {
    type Target = u8;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.bits
    }
}
#[doc = "The MAC command identifier."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IdA {
    #[doc = "Association request command"]
    AssocRequest = 1,
    #[doc = "Association response command"]
    AssocResponse = 2,
    #[doc = "Dissassociation Notification command"]
    DissasocNotify = 3,
    #[doc = "Data request command"]
    DataRequest = 4,
    #[doc = "PAN ID Conflict Notification command"]
    PanIdConflict = 5,
    #[doc = "Orphan notification command"]
    OrphanNotify = 6,
    #[doc = "Beacon request command"]
    BeaconRequest = 7,
    #[doc = "Coordinator Realignment command"]
    CoordinatorRealign = 8,
    #[doc = "GTS request command"]
    GtsRequest = 9,
    #[doc = "TRLE Management Request command"]
    TrleMgmtRequest = 10,
    #[doc = "TRLE Management Response command"]
    TrleMgmtResponse = 11,
    #[doc = "DSME Association Request command"]
    DsmeAssociationRequest = 19,
    #[doc = "DSME Association Response command"]
    DsmeAssociationResponse = 20,
    #[doc = "DSME Association Request command"]
    DsmeGtsRequest = 21,
    #[doc = "DSME Association Response command"]
    DsmeGtsResponse = 22,
    #[doc = "DSME Association Notify command"]
    DsmeGtsNotify = 23,
    #[doc = "DSME Information Request command"]
    DsmeInfoRequest = 24,
    #[doc = "DSME Information Response command"]
    DsmeInfoResponse = 25,
    #[doc = "DSME Beacon Allocation Notification command"]
    DsmeBeaconAllocNotify = 26,
    #[doc = "DSME Beacon Collision Notification command"]
    DsmeBeaconCollisionNotify = 27,
    #[doc = "DSME Link Report command"]
    DsmeLinkReport = 28,
    #[doc = "RIT Data Request command"]
    RitDataRequest = 32,
    #[doc = "DBS Request command"]
    DbsRequest = 33,
    #[doc = "DBS Response command"]
    DbsResponse = 34,
    #[doc = "RIT Data Response command"]
    RitDataResponse = 35,
    #[doc = "Vendor Specific command"]
    VendorSpecific = 36,
    #[doc = "SRM Request command"]
    SrmRequest = 37,
    #[doc = "SRM Response command"]
    SrmResponse = 38,
    #[doc = "SRM Report command"]
    SrmReport = 39,
    #[doc = "SRM Information command"]
    SrmInfo = 40,
}
impl From<IdA> for u8 {
    #[inline(always)]
    fn from(variant: IdA) -> Self {
        variant as _
    }
}
#[doc = "Field `Id` reader - The MAC command identifier."]
pub struct IdR(crate::FieldReader<u8, IdA>);
impl IdR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IdR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> IdA {
        match self.bits {
            1 => IdA::AssocRequest,
            2 => IdA::AssocResponse,
            3 => IdA::DissasocNotify,
            4 => IdA::DataRequest,
            5 => IdA::PanIdConflict,
            6 => IdA::OrphanNotify,
            7 => IdA::BeaconRequest,
            8 => IdA::CoordinatorRealign,
            9 => IdA::GtsRequest,
            10 => IdA::TrleMgmtRequest,
            11 => IdA::TrleMgmtResponse,
            19 => IdA::DsmeAssociationRequest,
            20 => IdA::DsmeAssociationResponse,
            21 => IdA::DsmeGtsRequest,
            22 => IdA::DsmeGtsResponse,
            23 => IdA::DsmeGtsNotify,
            24 => IdA::DsmeInfoRequest,
            25 => IdA::DsmeInfoResponse,
            26 => IdA::DsmeBeaconAllocNotify,
            27 => IdA::DsmeBeaconCollisionNotify,
            28 => IdA::DsmeLinkReport,
            32 => IdA::RitDataRequest,
            33 => IdA::DbsRequest,
            34 => IdA::DbsResponse,
            35 => IdA::RitDataResponse,
            36 => IdA::VendorSpecific,
            37 => IdA::SrmRequest,
            38 => IdA::SrmResponse,
            39 => IdA::SrmReport,
            40 => IdA::SrmInfo,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the `Id` field is `AssocRequest`"]
    #[inline(always)]
    pub fn is_assoc_request(&self) -> bool {
        **self == IdA::AssocRequest
    }
    #[doc = "Checks if the value of the `Id` field is `AssocResponse`"]
    #[inline(always)]
    pub fn is_assoc_response(&self) -> bool {
        **self == IdA::AssocResponse
    }
    #[doc = "Checks if the value of the `Id` field is `DissasocNotify`"]
    #[inline(always)]
    pub fn is_dissasoc_notify(&self) -> bool {
        **self == IdA::DissasocNotify
    }
    #[doc = "Checks if the value of the `Id` field is `DataRequest`"]
    #[inline(always)]
    pub fn is_data_request(&self) -> bool {
        **self == IdA::DataRequest
    }
    #[doc = "Checks if the value of the `Id` field is `PanIdConflict`"]
    #[inline(always)]
    pub fn is_pan_id_conflict(&self) -> bool {
        **self == IdA::PanIdConflict
    }
    #[doc = "Checks if the value of the `Id` field is `OrphanNotify`"]
    #[inline(always)]
    pub fn is_orphan_notify(&self) -> bool {
        **self == IdA::OrphanNotify
    }
    #[doc = "Checks if the value of the `Id` field is `BeaconRequest`"]
    #[inline(always)]
    pub fn is_beacon_request(&self) -> bool {
        **self == IdA::BeaconRequest
    }
    #[doc = "Checks if the value of the `Id` field is `CoordinatorRealign`"]
    #[inline(always)]
    pub fn is_coordinator_realign(&self) -> bool {
        **self == IdA::CoordinatorRealign
    }
    #[doc = "Checks if the value of the `Id` field is `GtsRequest`"]
    #[inline(always)]
    pub fn is_gts_request(&self) -> bool {
        **self == IdA::GtsRequest
    }
    #[doc = "Checks if the value of the `Id` field is `TrleMgmtRequest`"]
    #[inline(always)]
    pub fn is_trle_mgmt_request(&self) -> bool {
        **self == IdA::TrleMgmtRequest
    }
    #[doc = "Checks if the value of the `Id` field is `TrleMgmtResponse`"]
    #[inline(always)]
    pub fn is_trle_mgmt_response(&self) -> bool {
        **self == IdA::TrleMgmtResponse
    }
    #[doc = "Checks if the value of the `Id` field is `DsmeAssociationRequest`"]
    #[inline(always)]
    pub fn is_dsme_association_request(&self) -> bool {
        **self == IdA::DsmeAssociationRequest
    }
    #[doc = "Checks if the value of the `Id` field is `DsmeAssociationResponse`"]
    #[inline(always)]
    pub fn is_dsme_association_response(&self) -> bool {
        **self == IdA::DsmeAssociationResponse
    }
    #[doc = "Checks if the value of the `Id` field is `DsmeGtsRequest`"]
    #[inline(always)]
    pub fn is_dsme_gts_request(&self) -> bool {
        **self == IdA::DsmeGtsRequest
    }
    #[doc = "Checks if the value of the `Id` field is `DsmeGtsResponse`"]
    #[inline(always)]
    pub fn is_dsme_gts_response(&self) -> bool {
        **self == IdA::DsmeGtsResponse
    }
    #[doc = "Checks if the value of the `Id` field is `DsmeGtsNotify`"]
    #[inline(always)]
    pub fn is_dsme_gts_notify(&self) -> bool {
        **self == IdA::DsmeGtsNotify
    }
    #[doc = "Checks if the value of the `Id` field is `DsmeInfoRequest`"]
    #[inline(always)]
    pub fn is_dsme_info_request(&self) -> bool {
        **self == IdA::DsmeInfoRequest
    }
    #[doc = "Checks if the value of the `Id` field is `DsmeInfoResponse`"]
    #[inline(always)]
    pub fn is_dsme_info_response(&self) -> bool {
        **self == IdA::DsmeInfoResponse
    }
    #[doc = "Checks if the value of the `Id` field is `DsmeBeaconAllocNotify`"]
    #[inline(always)]
    pub fn is_dsme_beacon_alloc_notify(&self) -> bool {
        **self == IdA::DsmeBeaconAllocNotify
    }
    #[doc = "Checks if the value of the `Id` field is `DsmeBeaconCollisionNotify`"]
    #[inline(always)]
    pub fn is_dsme_beacon_collision_notify(&self) -> bool {
        **self == IdA::DsmeBeaconCollisionNotify
    }
    #[doc = "Checks if the value of the `Id` field is `DsmeLinkReport`"]
    #[inline(always)]
    pub fn is_dsme_link_report(&self) -> bool {
        **self == IdA::DsmeLinkReport
    }
    #[doc = "Checks if the value of the `Id` field is `RitDataRequest`"]
    #[inline(always)]
    pub fn is_rit_data_request(&self) -> bool {
        **self == IdA::RitDataRequest
    }
    #[doc = "Checks if the value of the `Id` field is `DbsRequest`"]
    #[inline(always)]
    pub fn is_dbs_request(&self) -> bool {
        **self == IdA::DbsRequest
    }
    #[doc = "Checks if the value of the `Id` field is `DbsResponse`"]
    #[inline(always)]
    pub fn is_dbs_response(&self) -> bool {
        **self == IdA::DbsResponse
    }
    #[doc = "Checks if the value of the `Id` field is `RitDataResponse`"]
    #[inline(always)]
    pub fn is_rit_data_response(&self) -> bool {
        **self == IdA::RitDataResponse
    }
    #[doc = "Checks if the value of the `Id` field is `VendorSpecific`"]
    #[inline(always)]
    pub fn is_vendor_specific(&self) -> bool {
        **self == IdA::VendorSpecific
    }
    #[doc = "Checks if the value of the `Id` field is `SrmRequest`"]
    #[inline(always)]
    pub fn is_srm_request(&self) -> bool {
        **self == IdA::SrmRequest
    }
    #[doc = "Checks if the value of the `Id` field is `SrmResponse`"]
    #[inline(always)]
    pub fn is_srm_response(&self) -> bool {
        **self == IdA::SrmResponse
    }
    #[doc = "Checks if the value of the `Id` field is `SrmReport`"]
    #[inline(always)]
    pub fn is_srm_report(&self) -> bool {
        **self == IdA::SrmReport
    }
    #[doc = "Checks if the value of the `Id` field is `SrmInfo`"]
    #[inline(always)]
    pub fn is_srm_info(&self) -> bool {
        **self == IdA::SrmInfo
    }
}
impl core::ops::Deref for IdR {
    type Target = crate::FieldReader<u8, IdA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct IdW<'a> {
    w: &'a mut W,
}
impl<'a> IdW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: IdA) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set the value of the `Id` field to `AssocRequest`"]
    #[inline(always)]
    pub fn assoc_request(self) -> &'a mut W {
        self.variant(IdA::AssocRequest)
    }
    #[doc = "Set the value of the `Id` field to `AssocResponse`"]
    #[inline(always)]
    pub fn assoc_response(self) -> &'a mut W {
        self.variant(IdA::AssocResponse)
    }
    #[doc = "Set the value of the `Id` field to `DissasocNotify`"]
    #[inline(always)]
    pub fn dissasoc_notify(self) -> &'a mut W {
        self.variant(IdA::DissasocNotify)
    }
    #[doc = "Set the value of the `Id` field to `DataRequest`"]
    #[inline(always)]
    pub fn data_request(self) -> &'a mut W {
        self.variant(IdA::DataRequest)
    }
    #[doc = "Set the value of the `Id` field to `PanIdConflict`"]
    #[inline(always)]
    pub fn pan_id_conflict(self) -> &'a mut W {
        self.variant(IdA::PanIdConflict)
    }
    #[doc = "Set the value of the `Id` field to `OrphanNotify`"]
    #[inline(always)]
    pub fn orphan_notify(self) -> &'a mut W {
        self.variant(IdA::OrphanNotify)
    }
    #[doc = "Set the value of the `Id` field to `BeaconRequest`"]
    #[inline(always)]
    pub fn beacon_request(self) -> &'a mut W {
        self.variant(IdA::BeaconRequest)
    }
    #[doc = "Set the value of the `Id` field to `CoordinatorRealign`"]
    #[inline(always)]
    pub fn coordinator_realign(self) -> &'a mut W {
        self.variant(IdA::CoordinatorRealign)
    }
    #[doc = "Set the value of the `Id` field to `GtsRequest`"]
    #[inline(always)]
    pub fn gts_request(self) -> &'a mut W {
        self.variant(IdA::GtsRequest)
    }
    #[doc = "Set the value of the `Id` field to `TrleMgmtRequest`"]
    #[inline(always)]
    pub fn trle_mgmt_request(self) -> &'a mut W {
        self.variant(IdA::TrleMgmtRequest)
    }
    #[doc = "Set the value of the `Id` field to `TrleMgmtResponse`"]
    #[inline(always)]
    pub fn trle_mgmt_response(self) -> &'a mut W {
        self.variant(IdA::TrleMgmtResponse)
    }
    #[doc = "Set the value of the `Id` field to `DsmeAssociationRequest`"]
    #[inline(always)]
    pub fn dsme_association_request(self) -> &'a mut W {
        self.variant(IdA::DsmeAssociationRequest)
    }
    #[doc = "Set the value of the `Id` field to `DsmeAssociationResponse`"]
    #[inline(always)]
    pub fn dsme_association_response(self) -> &'a mut W {
        self.variant(IdA::DsmeAssociationResponse)
    }
    #[doc = "Set the value of the `Id` field to `DsmeGtsRequest`"]
    #[inline(always)]
    pub fn dsme_gts_request(self) -> &'a mut W {
        self.variant(IdA::DsmeGtsRequest)
    }
    #[doc = "Set the value of the `Id` field to `DsmeGtsResponse`"]
    #[inline(always)]
    pub fn dsme_gts_response(self) -> &'a mut W {
        self.variant(IdA::DsmeGtsResponse)
    }
    #[doc = "Set the value of the `Id` field to `DsmeGtsNotify`"]
    #[inline(always)]
    pub fn dsme_gts_notify(self) -> &'a mut W {
        self.variant(IdA::DsmeGtsNotify)
    }
    #[doc = "Set the value of the `Id` field to `DsmeInfoRequest`"]
    #[inline(always)]
    pub fn dsme_info_request(self) -> &'a mut W {
        self.variant(IdA::DsmeInfoRequest)
    }
    #[doc = "Set the value of the `Id` field to `DsmeInfoResponse`"]
    #[inline(always)]
    pub fn dsme_info_response(self) -> &'a mut W {
        self.variant(IdA::DsmeInfoResponse)
    }
    #[doc = "Set the value of the `Id` field to `DsmeBeaconAllocNotify`"]
    #[inline(always)]
    pub fn dsme_beacon_alloc_notify(self) -> &'a mut W {
        self.variant(IdA::DsmeBeaconAllocNotify)
    }
    #[doc = "Set the value of the `Id` field to `DsmeBeaconCollisionNotify`"]
    #[inline(always)]
    pub fn dsme_beacon_collision_notify(self) -> &'a mut W {
        self.variant(IdA::DsmeBeaconCollisionNotify)
    }
    #[doc = "Set the value of the `Id` field to `DsmeLinkReport`"]
    #[inline(always)]
    pub fn dsme_link_report(self) -> &'a mut W {
        self.variant(IdA::DsmeLinkReport)
    }
    #[doc = "Set the value of the `Id` field to `RitDataRequest`"]
    #[inline(always)]
    pub fn rit_data_request(self) -> &'a mut W {
        self.variant(IdA::RitDataRequest)
    }
    #[doc = "Set the value of the `Id` field to `DbsRequest`"]
    #[inline(always)]
    pub fn dbs_request(self) -> &'a mut W {
        self.variant(IdA::DbsRequest)
    }
    #[doc = "Set the value of the `Id` field to `DbsResponse`"]
    #[inline(always)]
    pub fn dbs_response(self) -> &'a mut W {
        self.variant(IdA::DbsResponse)
    }
    #[doc = "Set the value of the `Id` field to `RitDataResponse`"]
    #[inline(always)]
    pub fn rit_data_response(self) -> &'a mut W {
        self.variant(IdA::RitDataResponse)
    }
    #[doc = "Set the value of the `Id` field to `VendorSpecific`"]
    #[inline(always)]
    pub fn vendor_specific(self) -> &'a mut W {
        self.variant(IdA::VendorSpecific)
    }
    #[doc = "Set the value of the `Id` field to `SrmRequest`"]
    #[inline(always)]
    pub fn srm_request(self) -> &'a mut W {
        self.variant(IdA::SrmRequest)
    }
    #[doc = "Set the value of the `Id` field to `SrmResponse`"]
    #[inline(always)]
    pub fn srm_response(self) -> &'a mut W {
        self.variant(IdA::SrmResponse)
    }
    #[doc = "Set the value of the `Id` field to `SrmReport`"]
    #[inline(always)]
    pub fn srm_report(self) -> &'a mut W {
        self.variant(IdA::SrmReport)
    }
    #[doc = "Set the value of the `Id` field to `SrmInfo`"]
    #[inline(always)]
    pub fn srm_info(self) -> &'a mut W {
        self.variant(IdA::SrmInfo)
    }
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 0)) | ((value as u8 & 0xff) << 0);
        self.w
    }
}
impl R {
    #[inline(always)]
    pub fn new(bits: u8) -> Self {
        R { bits }
    }
    #[doc = "Read the `Id` field."]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(((self.bits >> 0) & 0xff) as u8)
    }
}
impl W {
    #[inline(always)]
    pub fn new(bits: u8) -> Self {
        W { bits }
    }
    #[doc = "Set the `Id` field."]
    #[inline(always)]
    pub fn id(&mut self) -> IdW {
        IdW { w: self }
    }
}
