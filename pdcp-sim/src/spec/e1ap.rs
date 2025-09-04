#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod e1_ap_common_data_types {
    extern crate alloc;
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Common Data Types"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum Criticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum Presence {
        optional = 0,
        conditional = 1,
        mandatory = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags, identifier = "PrivateIE-ID")]
    pub enum PrivateIEID {
        #[rasn(value("0..=65535"))]
        local(u16),
        global(ObjectIdentifier),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct ProcedureCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct ProtocolExtensionID(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "ProtocolIE-ID", value("0..=65535"))]
    pub struct ProtocolIEID(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum TriggeringMessage {
        #[rasn(identifier = "initiating-message")]
        initiating_message = 0,
        #[rasn(identifier = "successful-outcome")]
        successful_outcome = 1,
        #[rasn(identifier = "unsuccessful-outcome")]
        unsuccessful_outcome = 2,
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Extension constants"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    pub static MAX_PRIVATE_IES: LazyLock<Integer> = LazyLock::new(|| Integer::from(65535));
    pub static MAX_PROTOCOL_EXTENSIONS: LazyLock<Integer> = LazyLock::new(|| Integer::from(65535));
    pub static MAX_PROTOCOL_IES: LazyLock<Integer> = LazyLock::new(|| Integer::from(65535));
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod e1_ap_constants {
    extern crate alloc;
    use super::e1_ap_common_data_types::{ProcedureCode, ProtocolIEID};
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;
    pub const ID_ACTIVITY_INFORMATION: ProtocolIEID = ProtocolIEID(24);
    pub const ID_ACTIVITY_NOTIFICATION_LEVEL: ProtocolIEID = ProtocolIEID(23);
    pub const ID_ADDITIONAL_HANDOVER_INFO: ProtocolIEID = ProtocolIEID(134);
    pub const ID_ADDITIONAL_PDCPDUPLICATION_INFORMATION: ProtocolIEID = ProtocolIEID(104);
    pub const ID_ADDITIONAL_RRMPRIORITY_INDEX: ProtocolIEID = ProtocolIEID(84);
    pub const ID_ALTERNATIVE_QO_SPARA_SET_LIST: ProtocolIEID = ProtocolIEID(124);
    pub const ID_ASSOCIATED_SESSION_ID: ProtocolIEID = ProtocolIEID(199);
    pub const ID_BCBEARER_CONTEXT_MODIFICATION: ProcedureCode = ProcedureCode(30);
    pub const ID_BCBEARER_CONTEXT_MODIFICATION_REQUIRED: ProcedureCode = ProcedureCode(31);
    pub const ID_BCBEARER_CONTEXT_NGU_TNLINFOAT_NGRAN_REQUEST: ProtocolIEID = ProtocolIEID(217);
    pub const ID_BCBEARER_CONTEXT_RELEASE: ProcedureCode = ProcedureCode(32);
    pub const ID_BCBEARER_CONTEXT_RELEASE_REQUEST: ProcedureCode = ProcedureCode(33);
    pub const ID_BCBEARER_CONTEXT_SETUP: ProcedureCode = ProcedureCode(29);
    pub const ID_BCBEARER_CONTEXT_TO_MODIFY: ProtocolIEID = ProtocolIEID(160);
    pub const ID_BCBEARER_CONTEXT_TO_MODIFY_CONFIRM: ProtocolIEID = ProtocolIEID(163);
    pub const ID_BCBEARER_CONTEXT_TO_MODIFY_REQUIRED: ProtocolIEID = ProtocolIEID(162);
    pub const ID_BCBEARER_CONTEXT_TO_MODIFY_RESPONSE: ProtocolIEID = ProtocolIEID(161);
    pub const ID_BCBEARER_CONTEXT_TO_SETUP: ProtocolIEID = ProtocolIEID(158);
    pub const ID_BCBEARER_CONTEXT_TO_SETUP_RESPONSE: ProtocolIEID = ProtocolIEID(159);
    pub const ID_BEARER_CONTEXT_STATUS_CHANGE: ProtocolIEID = ProtocolIEID(17);
    pub const ID_BROADCAST_F1_U_CONTEXT_REFERENCE_E1: ProtocolIEID = ProtocolIEID(213);
    pub const ID_CHOINITIATION: ProtocolIEID = ProtocolIEID(121);
    pub const ID_CNPACKET_DELAY_BUDGET_DOWNLINK: ProtocolIEID = ProtocolIEID(101);
    pub const ID_CNPACKET_DELAY_BUDGET_UPLINK: ProtocolIEID = ProtocolIEID(102);
    pub const ID_CNSUPPORT: ProtocolIEID = ProtocolIEID(10);
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " IEs"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    pub const ID_CAUSE: ProtocolIEID = ProtocolIEID(0);
    pub const ID_CELL_TRAFFIC_TRACE: ProcedureCode = ProcedureCode(25);
    pub const ID_COMMON_NETWORK_INSTANCE: ProtocolIEID = ProtocolIEID(78);
    pub const ID_CRITICALITY_DIAGNOSTICS: ProtocolIEID = ProtocolIEID(1);
    pub const ID_DAPSREQUEST_INFO: ProtocolIEID = ProtocolIEID(120);
    pub const ID_DLUPTNLADDRESS_TO_UPDATE_LIST: ProtocolIEID = ProtocolIEID(108);
    pub const ID_DRB_CONFIRM_MODIFIED_LIST_EUTRAN: ProtocolIEID = ProtocolIEID(41);
    pub const ID_DRB_FAILED_LIST_EUTRAN: ProtocolIEID = ProtocolIEID(38);
    pub const ID_DRB_FAILED_MOD_LIST_EUTRAN: ProtocolIEID = ProtocolIEID(53);
    pub const ID_DRB_FAILED_TO_MODIFY_LIST_EUTRAN: ProtocolIEID = ProtocolIEID(40);
    pub const ID_DRB_MEASUREMENT_RESULTS_INFORMATION_LIST: ProtocolIEID = ProtocolIEID(128);
    pub const ID_DRB_MODIFIED_LIST_EUTRAN: ProtocolIEID = ProtocolIEID(39);
    pub const ID_DRB_QO_S: ProtocolIEID = ProtocolIEID(72);
    pub const ID_DRB_REQUIRED_TO_MODIFY_LIST_EUTRAN: ProtocolIEID = ProtocolIEID(35);
    pub const ID_DRB_REQUIRED_TO_REMOVE_LIST_EUTRAN: ProtocolIEID = ProtocolIEID(36);
    pub const ID_DRB_SETUP_LIST_EUTRAN: ProtocolIEID = ProtocolIEID(37);
    pub const ID_DRB_SETUP_MOD_LIST_EUTRAN: ProtocolIEID = ProtocolIEID(52);
    pub const ID_DRB_STATUS_LIST: ProtocolIEID = ProtocolIEID(22);
    pub const ID_DRB_TO_MODIFY_LIST_EUTRAN: ProtocolIEID = ProtocolIEID(33);
    pub const ID_DRB_TO_REMOVE_LIST_EUTRAN: ProtocolIEID = ProtocolIEID(34);
    pub const ID_DRB_TO_SETUP_LIST_EUTRAN: ProtocolIEID = ProtocolIEID(32);
    pub const ID_DRB_TO_SETUP_MOD_LIST_EUTRAN: ProtocolIEID = ProtocolIEID(51);
    pub const ID_DRBS_SUBJECT_TO_COUNTER_CHECK_LIST_EUTRAN: ProtocolIEID = ProtocolIEID(61);
    pub const ID_DRBS_SUBJECT_TO_COUNTER_CHECK_LIST_NG_RAN: ProtocolIEID = ProtocolIEID(62);
    pub const ID_DRBS_SUBJECT_TO_EARLY_FORWARDING_LIST: ProtocolIEID = ProtocolIEID(119);
    pub const ID_DATA_USAGE_REPORT_LIST: ProtocolIEID = ProtocolIEID(25);
    pub const ID_DATA_DISCARD_REQUIRED: ProtocolIEID = ProtocolIEID(70);
    pub const ID_DATA_FORWARDING_SOURCE_IPADDRESS: ProtocolIEID = ProtocolIEID(142);
    pub const ID_DATA_FORWARDINGTO_E_UTRANINFORMATION_LIST: ProtocolIEID = ProtocolIEID(131);
    pub const ID_DATA_FORWARDINGTO_NG_RANQO_SFLOW_INFORMATION_LIST: ProtocolIEID =
        ProtocolIEID(136);
    pub const ID_DEACTIVATE_TRACE: ProcedureCode = ProcedureCode(21);
    pub const ID_DIRECT_FORWARDING_PATH_AVAILABILITY: ProtocolIEID = ProtocolIEID(139);
    pub const ID_DISCARD_TIMER_EXTENDED: ProtocolIEID = ProtocolIEID(177);
    pub const ID_ECGI_SUPPORT_LIST: ProtocolIEID = ProtocolIEID(145);
    pub const ID_ECNMARKINGOR_CONGESTION_INFORMATION_REPORTING_REQUEST: ProtocolIEID =
        ProtocolIEID(203);
    pub const ID_ECNMARKINGOR_CONGESTION_INFORMATION_REPORTING_STATUS: ProtocolIEID =
        ProtocolIEID(204);
    pub const ID_EHC_PARAMETERS: ProtocolIEID = ProtocolIEID(118);
    pub const ID_EARLY_DATA_FORWARDING_INDICATOR: ProtocolIEID = ProtocolIEID(140);
    pub const ID_EARLY_FORWARDING_COUNTINFO: ProtocolIEID = ProtocolIEID(123);
    pub const ID_EARLY_FORWARDING_COUNTREQ: ProtocolIEID = ProtocolIEID(122);
    pub const ID_EXTENDED_GNB_CU_CP_NAME: ProtocolIEID = ProtocolIEID(129);
    pub const ID_EXTENDED_GNB_CU_UP_NAME: ProtocolIEID = ProtocolIEID(130);
    pub const ID_EXTENDED_NR_CGI_SUPPORT_LIST: ProtocolIEID = ProtocolIEID(135);
    pub const ID_EXTENDED_PACKET_DELAY_BUDGET: ProtocolIEID = ProtocolIEID(103);
    pub const ID_EXTENDED_SLICE_SUPPORT_LIST: ProtocolIEID = ProtocolIEID(125);
    pub const ID_F1_U_TNL_INFO_ADDED_LIST: ProtocolIEID = ProtocolIEID(209);
    pub const ID_F1_U_TNL_INFO_ADDED_OR_MODIFIED_LIST: ProtocolIEID = ProtocolIEID(211);
    pub const ID_F1_U_TNL_INFO_TO_ADD_LIST: ProtocolIEID = ProtocolIEID(208);
    pub const ID_F1_U_TNL_INFO_TO_ADD_OR_MODIFY_LIST: ProtocolIEID = ProtocolIEID(210);
    pub const ID_F1_U_TNL_INFO_TO_RELEASE_LIST: ProtocolIEID = ProtocolIEID(212);
    pub const ID_F1_UTUNNEL_NOT_ESTABLISHED: ProtocolIEID = ProtocolIEID(207);
    pub const ID_GNB_CU_CP_MBS_E1_AP_ID: ProtocolIEID = ProtocolIEID(155);
    pub const ID_GNB_CU_CP_TNLA_FAILED_TO_SETUP_LIST: ProtocolIEID = ProtocolIEID(31);
    pub const ID_GNB_CU_CP_TNLA_SETUP_LIST: ProtocolIEID = ProtocolIEID(30);
    pub const ID_GNB_CU_CP_TNLA_TO_ADD_LIST: ProtocolIEID = ProtocolIEID(27);
    pub const ID_GNB_CU_CP_TNLA_TO_REMOVE_LIST: ProtocolIEID = ProtocolIEID(28);
    pub const ID_GNB_CU_CP_TNLA_TO_UPDATE_LIST: ProtocolIEID = ProtocolIEID(29);
    pub const ID_GNB_CU_UP_MBS_E1_AP_ID: ProtocolIEID = ProtocolIEID(156);
    pub const ID_GNB_CU_UP_OVERLOAD_INFORMATION: ProtocolIEID = ProtocolIEID(65);
    pub const ID_GNB_CU_UP_TNLA_TO_REMOVE_LIST: ProtocolIEID = ProtocolIEID(73);
    pub const ID_GNB_DU_ID: ProtocolIEID = ProtocolIEID(77);
    pub const ID_GLOBAL_MBSSESSION_ID: ProtocolIEID = ProtocolIEID(157);
    pub const ID_HW_CAPACITY_INDICATOR: ProtocolIEID = ProtocolIEID(95);
    pub const ID_IAB_DONOR_CU_UPPSKINFO: ProtocolIEID = ProtocolIEID(144);
    pub const ID_INACTIVITY_INFORMATION_REQUEST: ProtocolIEID = ProtocolIEID(187);
    pub const ID_INDIRECT_PATH_INDICATION: ProtocolIEID = ProtocolIEID(206);
    pub const ID_M4_REPORT_AMOUNT: ProtocolIEID = ProtocolIEID(147);
    pub const ID_M6_REPORT_AMOUNT: ProtocolIEID = ProtocolIEID(148);
    pub const ID_M7_REPORT_AMOUNT: ProtocolIEID = ProtocolIEID(149);
    pub const ID_MBS_SERVICE_AREA: ProtocolIEID = ProtocolIEID(200);
    pub const ID_MBSAREA_SESSION_ID: ProtocolIEID = ProtocolIEID(189);
    pub const ID_MBSMULTICAST_F1_UCONTEXT_DESCRIPTOR: ProtocolIEID = ProtocolIEID(170);
    pub const ID_MBSSESSION_ASSOCIATED_INFO_NON_SUPPORT_TO_SUPPORT: ProtocolIEID =
        ProtocolIEID(185);
    pub const ID_MBSSESSION_RESOURCE_NOTIFICATION: ProtocolIEID = ProtocolIEID(191);
    pub const ID_MCBEARER_CONTEXT_INACTIVITY_TIMER: ProtocolIEID = ProtocolIEID(192);
    pub const ID_MCBEARER_CONTEXT_MODIFICATION: ProcedureCode = ProcedureCode(35);
    pub const ID_MCBEARER_CONTEXT_MODIFICATION_REQUIRED: ProcedureCode = ProcedureCode(36);
    pub const ID_MCBEARER_CONTEXT_RELEASE: ProcedureCode = ProcedureCode(37);
    pub const ID_MCBEARER_CONTEXT_RELEASE_REQUEST: ProcedureCode = ProcedureCode(38);
    pub const ID_MCBEARER_CONTEXT_SETUP: ProcedureCode = ProcedureCode(34);
    pub const ID_MCBEARER_CONTEXT_STATUS_CHANGE: ProtocolIEID = ProtocolIEID(193);
    pub const ID_MCBEARER_CONTEXT_TO_MODIFY: ProtocolIEID = ProtocolIEID(166);
    pub const ID_MCBEARER_CONTEXT_TO_MODIFY_CONFIRM: ProtocolIEID = ProtocolIEID(169);
    pub const ID_MCBEARER_CONTEXT_TO_MODIFY_REQUIRED: ProtocolIEID = ProtocolIEID(168);
    pub const ID_MCBEARER_CONTEXT_TO_MODIFY_RESPONSE: ProtocolIEID = ProtocolIEID(167);
    pub const ID_MCBEARER_CONTEXT_TO_SETUP: ProtocolIEID = ProtocolIEID(164);
    pub const ID_MCBEARER_CONTEXT_TO_SETUP_RESPONSE: ProtocolIEID = ProtocolIEID(165);
    pub const ID_MCBEARER_NOTIFICATION: ProcedureCode = ProcedureCode(39);
    pub const ID_MCFORWARDING_RESOURCE_INDICATION: ProtocolIEID = ProtocolIEID(180);
    pub const ID_MCFORWARDING_RESOURCE_RELEASE: ProtocolIEID = ProtocolIEID(182);
    pub const ID_MCFORWARDING_RESOURCE_RELEASE_INDICATION: ProtocolIEID = ProtocolIEID(183);
    pub const ID_MCFORWARDING_RESOURCE_REQUEST: ProtocolIEID = ProtocolIEID(179);
    pub const ID_MCFORWARDING_RESOURCE_RESPONSE: ProtocolIEID = ProtocolIEID(181);
    pub const ID_MCG_OFFERED_GBRQO_SFLOW_INFO: ProtocolIEID = ProtocolIEID(126);
    pub const ID_MDTCONFIGURATION: ProtocolIEID = ProtocolIEID(112);
    pub const ID_MDTPOLLUTED_MEASUREMENT_INDICATOR: ProtocolIEID = ProtocolIEID(146);
    pub const ID_MT_SDT_INFORMATION: ProtocolIEID = ProtocolIEID(194);
    pub const ID_MT_SDT_INFORMATION_REQUEST: ProtocolIEID = ProtocolIEID(195);
    pub const ID_MANAGEMENT_BASED_MDTPLMNLIST: ProtocolIEID = ProtocolIEID(113);
    pub const ID_MANAGEMENT_BASED_MDTPLMNMODIFICATION_LIST: ProtocolIEID = ProtocolIEID(178);
    pub const ID_MAX_CIDEHCDL: ProtocolIEID = ProtocolIEID(137);
    pub const ID_MAXIMUM_DATA_BURST_VOLUME: ProtocolIEID = ProtocolIEID(216);
    pub const ID_N6_JITTER_INFORMATION: ProtocolIEID = ProtocolIEID(202);
    pub const ID_NPNCONTEXT_INFO: ProtocolIEID = ProtocolIEID(111);
    pub const ID_NPNSUPPORT_INFO: ProtocolIEID = ProtocolIEID(110);
    pub const ID_NETWORK_INSTANCE: ProtocolIEID = ProtocolIEID(79);
    pub const ID_NEW_UL_TNL_INFORMATION_REQUIRED: ProtocolIEID = ProtocolIEID(26);
    pub const ID_NUMBER_OF_TUNNELS: ProtocolIEID = ProtocolIEID(127);
    pub const ID_OLD_QO_SFLOW_MAP_ULENDMARKEREXPECTED: ProtocolIEID = ProtocolIEID(71);
    pub const ID_PDCP_COUNT_RESET: ProtocolIEID = ProtocolIEID(184);
    pub const ID_PDCP_STATUS_REPORT_INDICATION: ProtocolIEID = ProtocolIEID(88);
    pub const ID_PDCPSNGAP_REPORT: ProtocolIEID = ProtocolIEID(218);
    pub const ID_PDU_SESSION_RESOURCE_CONFIRM_MODIFIED_LIST: ProtocolIEID = ProtocolIEID(50);
    pub const ID_PDU_SESSION_RESOURCE_DATA_USAGE_LIST: ProtocolIEID = ProtocolIEID(68);
    pub const ID_PDU_SESSION_RESOURCE_FAILED_LIST: ProtocolIEID = ProtocolIEID(47);
    pub const ID_PDU_SESSION_RESOURCE_FAILED_MOD_LIST: ProtocolIEID = ProtocolIEID(55);
    pub const ID_PDU_SESSION_RESOURCE_FAILED_TO_MODIFY_LIST: ProtocolIEID = ProtocolIEID(49);
    pub const ID_PDU_SESSION_RESOURCE_MODIFIED_LIST: ProtocolIEID = ProtocolIEID(48);
    pub const ID_PDU_SESSION_RESOURCE_REQUIRED_TO_MODIFY_LIST: ProtocolIEID = ProtocolIEID(45);
    pub const ID_PDU_SESSION_RESOURCE_SETUP_LIST: ProtocolIEID = ProtocolIEID(46);
    pub const ID_PDU_SESSION_RESOURCE_SETUP_MOD_LIST: ProtocolIEID = ProtocolIEID(54);
    pub const ID_PDU_SESSION_RESOURCE_TO_MODIFY_LIST: ProtocolIEID = ProtocolIEID(43);
    pub const ID_PDU_SESSION_RESOURCE_TO_REMOVE_LIST: ProtocolIEID = ProtocolIEID(44);
    pub const ID_PDU_SESSION_RESOURCE_TO_SETUP_LIST: ProtocolIEID = ProtocolIEID(42);
    pub const ID_PDU_SESSION_RESOURCE_TO_SETUP_MOD_LIST: ProtocolIEID = ProtocolIEID(56);
    pub const ID_PDU_SESSION_TO_NOTIFY_LIST: ProtocolIEID = ProtocolIEID(67);
    pub const ID_PDUSESSION_PAIR_ID: ProtocolIEID = ProtocolIEID(151);
    pub const ID_PDUSET_QO_SPARAMETERS: ProtocolIEID = ProtocolIEID(201);
    pub const ID_PDUSETBASED_HANDLING_INDICATOR: ProtocolIEID = ProtocolIEID(205);
    pub const ID_PPI: ProtocolIEID = ProtocolIEID(63);
    pub const ID_PSIBASED_DISCARD_TIMER: ProtocolIEID = ProtocolIEID(214);
    pub const ID_PRIVACY_INDICATOR: ProtocolIEID = ProtocolIEID(115);
    pub const ID_QO_S_MAPPING_INFORMATION: ProtocolIEID = ProtocolIEID(107);
    pub const ID_QO_SFLOW_MAPPING_INDICATION: ProtocolIEID = ProtocolIEID(80);
    pub const ID_QO_SFLOWS_DRBREMAPPING: ProtocolIEID = ProtocolIEID(141);
    pub const ID_QO_SMONITORING_DISABLED: ProtocolIEID = ProtocolIEID(133);
    pub const ID_QO_SMONITORING_REQUEST: ProtocolIEID = ProtocolIEID(87);
    pub const ID_QOS_MONITORING_REPORTING_FREQUENCY: ProtocolIEID = ProtocolIEID(132);
    pub const ID_RANUEID: ProtocolIEID = ProtocolIEID(76);
    pub const ID_REDUNDANT_COMMON_NETWORK_INSTANCE: ProtocolIEID = ProtocolIEID(96);
    pub const ID_REDUNDANT_PDUSESSION_INFORMATION: ProtocolIEID = ProtocolIEID(105);
    pub const ID_REDUNDANT_PDUSESSION_INFORMATION_USED: ProtocolIEID = ProtocolIEID(106);
    pub const ID_REDUNDANT_QOS_FLOW_INDICATOR: ProtocolIEID = ProtocolIEID(99);
    pub const ID_REGISTRATION_REQUEST: ProtocolIEID = ProtocolIEID(91);
    pub const ID_REPORT_CHARACTERISTICS: ProtocolIEID = ProtocolIEID(92);
    pub const ID_REPORTING_PERIODICITY: ProtocolIEID = ProtocolIEID(93);
    pub const ID_RESET_TYPE: ProtocolIEID = ProtocolIEID(4);
    pub const ID_RETAINABILITY_MEASUREMENTS_INFO: ProtocolIEID = ProtocolIEID(85);
    pub const ID_SCGACTIVATION_STATUS: ProtocolIEID = ProtocolIEID(154);
    pub const ID_SDT_DATA_SIZE_THRESHOLD: ProtocolIEID = ProtocolIEID(196);
    pub const ID_SDT_DATA_SIZE_THRESHOLD_CROSSED: ProtocolIEID = ProtocolIEID(197);
    pub const ID_SDTCONTINUE_ROHC: ProtocolIEID = ProtocolIEID(174);
    pub const ID_SDTINDICATOR_MOD: ProtocolIEID = ProtocolIEID(176);
    pub const ID_SDTINDICATOR_SETUP: ProtocolIEID = ProtocolIEID(175);
    pub const ID_SNSSAI: ProtocolIEID = ProtocolIEID(69);
    pub const ID_SECONDARY_PDU_SESSION_DATA_FORWARDING_INFORMATION: ProtocolIEID =
        ProtocolIEID(190);
    pub const ID_SECURITY_INDICATION: ProtocolIEID = ProtocolIEID(172);
    pub const ID_SECURITY_INDICATION_MODIFY: ProtocolIEID = ProtocolIEID(143);
    pub const ID_SECURITY_INFORMATION: ProtocolIEID = ProtocolIEID(13);
    pub const ID_SECURITY_RESULT: ProtocolIEID = ProtocolIEID(173);
    pub const ID_SERVING_PLMN: ProtocolIEID = ProtocolIEID(58);
    pub const ID_SPECIAL_TRIGGERING_PURPOSE: ProtocolIEID = ProtocolIEID(198);
    pub const ID_SUBSCRIBER_PROFILE_IDFOR_RFP: ProtocolIEID = ProtocolIEID(83);
    pub const ID_SUPPORTED_PLMNS: ProtocolIEID = ProtocolIEID(11);
    pub const ID_SURVIVAL_TIME: ProtocolIEID = ProtocolIEID(152);
    pub const ID_SYSTEM_BEARER_CONTEXT_MODIFICATION_CONFIRM: ProtocolIEID = ProtocolIEID(20);
    pub const ID_SYSTEM_BEARER_CONTEXT_MODIFICATION_REQUEST: ProtocolIEID = ProtocolIEID(18);
    pub const ID_SYSTEM_BEARER_CONTEXT_MODIFICATION_REQUIRED: ProtocolIEID = ProtocolIEID(21);
    pub const ID_SYSTEM_BEARER_CONTEXT_MODIFICATION_RESPONSE: ProtocolIEID = ProtocolIEID(19);
    pub const ID_SYSTEM_BEARER_CONTEXT_SETUP_REQUEST: ProtocolIEID = ProtocolIEID(15);
    pub const ID_SYSTEM_BEARER_CONTEXT_SETUP_RESPONSE: ProtocolIEID = ProtocolIEID(16);
    pub const ID_SYSTEM_GNB_CU_UP_COUNTER_CHECK_REQUEST: ProtocolIEID = ProtocolIEID(60);
    pub const ID_TNL_AVAILABLE_CAPACITY_INDICATOR: ProtocolIEID = ProtocolIEID(94);
    pub const ID_TNLASSOCIATION_TRANSPORT_LAYER_ADDRESSG_NBCUUP: ProtocolIEID = ProtocolIEID(75);
    pub const ID_TSCTRAFFIC_CHARACTERISTICS: ProtocolIEID = ProtocolIEID(100);
    pub const ID_TIME_TO_WAIT: ProtocolIEID = ProtocolIEID(12);
    pub const ID_TRACE_ACTIVATION: ProtocolIEID = ProtocolIEID(81);
    pub const ID_TRACE_COLLECTION_ENTITY_IPADDRESS: ProtocolIEID = ProtocolIEID(114);
    pub const ID_TRACE_COLLECTION_ENTITY_URI: ProtocolIEID = ProtocolIEID(116);
    pub const ID_TRACE_ID: ProtocolIEID = ProtocolIEID(82);
    pub const ID_TRACE_START: ProcedureCode = ProcedureCode(20);
    pub const ID_TRANSACTION_ID: ProtocolIEID = ProtocolIEID(57);
    pub const ID_TRANSPORT_LAYER_ADDRESS_INFO: ProtocolIEID = ProtocolIEID(86);
    pub const ID_UDC_PARAMETERS: ProtocolIEID = ProtocolIEID(153);
    pub const ID_UE_INACTIVITY_TIMER: ProtocolIEID = ProtocolIEID(59);
    pub const ID_UE_ASSOCIATED_LOGICAL_E1_CONNECTION_ITEM: ProtocolIEID = ProtocolIEID(5);
    pub const ID_UE_ASSOCIATED_LOGICAL_E1_CONNECTION_LIST_RES_ACK: ProtocolIEID = ProtocolIEID(6);
    pub const ID_UEDLAGGREGATE_MAXIMUM_BIT_RATE: ProtocolIEID = ProtocolIEID(14);
    pub const ID_UEDLMAXIMUM_INTEGRITY_PROTECTED_DATA_RATE: ProtocolIEID = ProtocolIEID(66);
    pub const ID_UEINACTIVITY_INFORMATION: ProtocolIEID = ProtocolIEID(188);
    pub const ID_UESLICE_MAXIMUM_BIT_RATE_LIST: ProtocolIEID = ProtocolIEID(150);
    pub const ID_ULUPTNLADDRESS_TO_UPDATE_LIST: ProtocolIEID = ProtocolIEID(109);
    pub const ID_URIADDRESS: ProtocolIEID = ProtocolIEID(117);
    pub const ID_USER_PLANE_ERROR_INDICATOR: ProtocolIEID = ProtocolIEID(215);
    pub const ID_USER_PLANE_FAILURE_INDICATION: ProtocolIEID = ProtocolIEID(219);
    pub const ID_VERSION_ID: ProtocolIEID = ProtocolIEID(186);
    pub const ID_BEARER_CONTEXT_INACTIVITY_NOTIFICATION: ProcedureCode = ProcedureCode(13);
    pub const ID_BEARER_CONTEXT_MODIFICATION: ProcedureCode = ProcedureCode(9);
    pub const ID_BEARER_CONTEXT_MODIFICATION_REQUIRED: ProcedureCode = ProcedureCode(10);
    pub const ID_BEARER_CONTEXT_RELEASE: ProcedureCode = ProcedureCode(11);
    pub const ID_BEARER_CONTEXT_RELEASE_REQUEST: ProcedureCode = ProcedureCode(12);
    pub const ID_BEARER_CONTEXT_SETUP: ProcedureCode = ProcedureCode(8);
    pub const ID_D_LDATA_NOTIFICATION: ProcedureCode = ProcedureCode(14);
    pub const ID_DATA_USAGE_REPORT: ProcedureCode = ProcedureCode(15);
    pub const ID_E1_RELEASE: ProcedureCode = ProcedureCode(7);
    pub const ID_EARLY_FORWARDING_SNTRANSFER: ProcedureCode = ProcedureCode(26);
    pub const ID_ENDPOINT_IP_ADDRESS_AND_PORT: ProtocolIEID = ProtocolIEID(74);
    pub const ID_ERROR_INDICATION: ProcedureCode = ProcedureCode(1);
    pub const ID_G_NB_CU_CP_CONFIGURATION_UPDATE: ProcedureCode = ProcedureCode(6);
    pub const ID_G_NB_CU_CP_E1_SETUP: ProcedureCode = ProcedureCode(4);
    pub const ID_G_NB_CU_CP_MEASUREMENT_ID: ProtocolIEID = ProtocolIEID(89);
    pub const ID_G_NB_CU_CP_NAME: ProtocolIEID = ProtocolIEID(9);
    pub const ID_G_NB_CU_CP_UE_E1_AP_ID: ProtocolIEID = ProtocolIEID(2);
    pub const ID_G_NB_CU_CPMEASUREMENT_RESULTS_INFORMATION: ProcedureCode = ProcedureCode(27);
    pub const ID_G_NB_CU_UP_CAPACITY: ProtocolIEID = ProtocolIEID(64);
    pub const ID_G_NB_CU_UP_CONFIGURATION_UPDATE: ProcedureCode = ProcedureCode(5);
    pub const ID_G_NB_CU_UP_COUNTER_CHECK: ProcedureCode = ProcedureCode(16);
    pub const ID_G_NB_CU_UP_E1_SETUP: ProcedureCode = ProcedureCode(3);
    pub const ID_G_NB_CU_UP_ID: ProtocolIEID = ProtocolIEID(7);
    pub const ID_G_NB_CU_UP_MBS_SUPPORT_INFO: ProtocolIEID = ProtocolIEID(171);
    pub const ID_G_NB_CU_UP_MEASUREMENT_ID: ProtocolIEID = ProtocolIEID(90);
    pub const ID_G_NB_CU_UP_NAME: ProtocolIEID = ProtocolIEID(8);
    pub const ID_G_NB_CU_UP_STATUS_INDICATION: ProcedureCode = ProcedureCode(17);
    pub const ID_G_NB_CU_UP_UE_E1_AP_ID: ProtocolIEID = ProtocolIEID(3);
    pub const ID_I_AB_UPTNLADDRESS_UPDATE: ProcedureCode = ProcedureCode(24);
    pub const ID_I_ABPSKNOTIFICATION: ProcedureCode = ProcedureCode(28);
    pub const ID_IGNORE_MAPPING_RULE_INDICATION: ProtocolIEID = ProtocolIEID(138);
    pub const ID_M_RDC_DATA_USAGE_REPORT: ProcedureCode = ProcedureCode(19);
    pub const ID_PRIVATE_MESSAGE: ProcedureCode = ProcedureCode(2);
    pub const ID_REDUNDANT_N_G_DL_UP_TNL_INFORMATION: ProtocolIEID = ProtocolIEID(98);
    pub const ID_REDUNDANT_N_G_UL_UP_TNL_INFORMATION: ProtocolIEID = ProtocolIEID(97);
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Elementary Procedures"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    pub const ID_RESET: ProcedureCode = ProcedureCode(0);
    pub const ID_RESOURCE_STATUS_REPORTING: ProcedureCode = ProcedureCode(23);
    pub const ID_RESOURCE_STATUS_REPORTING_INITIATION: ProcedureCode = ProcedureCode(22);
    pub const ID_U_LDATA_NOTIFICATION: ProcedureCode = ProcedureCode(18);
    pub static MAXNOOF_CELL_GROUPS: LazyLock<Integer> = LazyLock::new(|| Integer::from(4));
    pub static MAXNOOF_CELLSFOR_MBS: LazyLock<Integer> = LazyLock::new(|| Integer::from(512));
    pub static MAXNOOF_DRBS: LazyLock<Integer> = LazyLock::new(|| Integer::from(32));
    pub static MAXNOOF_DUS: LazyLock<Integer> = LazyLock::new(|| Integer::from(512));
    pub static MAXNOOF_DATA_FORWARDING_TUNNELTO_E_UTRAN: LazyLock<Integer> =
        LazyLock::new(|| Integer::from(256));
    pub static MAXNOOF_ECGI: LazyLock<Integer> = LazyLock::new(|| Integer::from(512));
    pub static MAXNOOF_EUTRANQOSPARAMETERS: LazyLock<Integer> =
        LazyLock::new(|| Integer::from(256));
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Lists"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    pub static MAXNOOF_ERRORS: LazyLock<Integer> = LazyLock::new(|| Integer::from(256));
    pub static MAXNOOF_EXT_NRCGI: LazyLock<Integer> = LazyLock::new(|| Integer::from(16384));
    pub static MAXNOOF_EXT_SLICE_ITEMS: LazyLock<Integer> = LazyLock::new(|| Integer::from(65535));
    pub static MAXNOOF_GTPTLAS: LazyLock<Integer> = LazyLock::new(|| Integer::from(16));
    pub static MAXNOOF_INDIVIDUAL_E1_CONNECTIONS_TO_RESET: LazyLock<Integer> =
        LazyLock::new(|| Integer::from(65536));
    pub static MAXNOOF_MBSAREA_SESSION_IDS: LazyLock<Integer> =
        LazyLock::new(|| Integer::from(256));
    pub static MAXNOOF_MBSSERVICE_AREA_INFORMATION: LazyLock<Integer> =
        LazyLock::new(|| Integer::from(256));
    pub static MAXNOOF_MBSSESSION_IDS: LazyLock<Integer> = LazyLock::new(|| Integer::from(512));
    pub static MAXNOOF_MDTPLMNS: LazyLock<Integer> = LazyLock::new(|| Integer::from(16));
    pub static MAXNOOF_MRBS: LazyLock<Integer> = LazyLock::new(|| Integer::from(32));
    pub static MAXNOOF_NGRANQOSPARAMETERS: LazyLock<Integer> = LazyLock::new(|| Integer::from(256));
    pub static MAXNOOF_NRCGI: LazyLock<Integer> = LazyLock::new(|| Integer::from(512));
    pub static MAXNOOF_PDUSESSION_RESOURCE: LazyLock<Integer> =
        LazyLock::new(|| Integer::from(256));
    pub static MAXNOOF_PSKS: LazyLock<Integer> = LazyLock::new(|| Integer::from(256));
    pub static MAXNOOF_QO_SFLOWS: LazyLock<Integer> = LazyLock::new(|| Integer::from(64));
    pub static MAXNOOF_QO_SPARA_SETS: LazyLock<Integer> = LazyLock::new(|| Integer::from(8));
    pub static MAXNOOF_SMBRVALUES: LazyLock<Integer> = LazyLock::new(|| Integer::from(8));
    pub static MAXNOOF_SPLMNS: LazyLock<Integer> = LazyLock::new(|| Integer::from(12));
    pub static MAXNOOF_SHARED_NG_UTERMINATIONS: LazyLock<Integer> =
        LazyLock::new(|| Integer::from(8));
    pub static MAXNOOF_SLICE_ITEMS: LazyLock<Integer> = LazyLock::new(|| Integer::from(1024));
    pub static MAXNOOF_TAIFOR_MBS: LazyLock<Integer> = LazyLock::new(|| Integer::from(512));
    pub static MAXNOOF_TLAS: LazyLock<Integer> = LazyLock::new(|| Integer::from(16));
    pub static MAXNOOF_TNLADDRESSES: LazyLock<Integer> = LazyLock::new(|| Integer::from(8));
    pub static MAXNOOF_TNLASSOCIATIONS: LazyLock<Integer> = LazyLock::new(|| Integer::from(32));
    pub static MAXNOOF_UPPARAMETERS: LazyLock<Integer> = LazyLock::new(|| Integer::from(8));
    pub static MAXNOOFTIMEPERIODS: LazyLock<Integer> = LazyLock::new(|| Integer::from(2));
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod e1_ap_containers {
    extern crate alloc;
    use super::e1_ap_common_data_types::{
        Criticality, Presence, PrivateIEID, ProtocolIEID, MAX_PRIVATE_IES, MAX_PROTOCOL_EXTENSIONS,
        MAX_PROTOCOL_IES,
    };
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Class Definition for Private IEs"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    pub trait E1APPRIVATEIES {
        const ID: PrivateIEID;
        const CRITICALITY: Criticality;
        type Value: rasn::prelude::AsnType;
        const PRESENCE: Presence;
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Class Definition for Protocol Extensions"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    pub trait E1APPROTOCOLEXTENSION {
        const ID: ProtocolIEID;
        const CRITICALITY: Criticality;
        type Extension: rasn::prelude::AsnType;
        const PRESENCE: Presence;
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Class Definition for Protocol IEs"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    pub trait E1APPROTOCOLIES {
        const ID: ProtocolIEID;
        const CRITICALITY: Criticality;
        type Value: rasn::prelude::AsnType;
        const PRESENCE: Presence;
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum AnonymousPrivateIEContainerId {
        #[rasn(value("0..=65535"))]
        local(u16),
        global(ObjectIdentifier),
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousPrivateIEContainerCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPrivateIEContainer {
        pub id: AnonymousPrivateIEContainerId,
        pub criticality: AnonymousPrivateIEContainerCriticality,
        pub value: Any,
    }
    impl AnonymousPrivateIEContainer {
        pub fn new(
            id: AnonymousPrivateIEContainerId,
            criticality: AnonymousPrivateIEContainerCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Container for Private IEs"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"), identifier = "PrivateIE-Container")]
    pub struct PrivateIEContainer(pub SequenceOf<AnonymousPrivateIEContainer>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "PrivateIE-Field")]
    pub struct PrivateIEField {
        pub id: PrivateIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl PrivateIEField {
        pub fn new(id: PrivateIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousProtocolExtensionContainerCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousProtocolExtensionContainer {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousProtocolExtensionContainerCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousProtocolExtensionContainer {
        pub fn new(
            id: u16,
            criticality: AnonymousProtocolExtensionContainerCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Container for Protocol Extensions"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct ProtocolExtensionContainer(pub SequenceOf<AnonymousProtocolExtensionContainer>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ProtocolExtensionField {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl ProtocolExtensionField {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousProtocolIEContainerCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousProtocolIEContainer {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousProtocolIEContainerCriticality,
        pub value: Any,
    }
    impl AnonymousProtocolIEContainer {
        pub fn new(
            id: u16,
            criticality: AnonymousProtocolIEContainerCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Container for Protocol IEs"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"), identifier = "ProtocolIE-Container")]
    pub struct ProtocolIEContainer(pub SequenceOf<AnonymousProtocolIEContainer>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousAnonymousProtocolIEContainerListCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousAnonymousProtocolIEContainerList {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousAnonymousProtocolIEContainerListCriticality,
        pub value: Any,
    }
    impl AnonymousAnonymousProtocolIEContainerList {
        pub fn new(
            id: u16,
            criticality: AnonymousAnonymousProtocolIEContainerListCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"), identifier = "SEQUENCE_OF")]
    pub struct AnonymousProtocolIEContainerList(
        pub SequenceOf<AnonymousAnonymousProtocolIEContainerList>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Container Lists for Protocol IE Containers"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "ProtocolIE-ContainerList")]
    pub struct ProtocolIEContainerList(pub SequenceOf<AnonymousProtocolIEContainerList>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "ProtocolIE-Field")]
    pub struct ProtocolIEField {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl ProtocolIEField {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "ProtocolIE-SingleContainer")]
    pub struct ProtocolIESingleContainer {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl ProtocolIESingleContainer {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod e1_ap_ies {
    extern crate alloc;
    use super::e1_ap_common_data_types::{
        Criticality, ProcedureCode, ProtocolIEID, TriggeringMessage,
    };
    use super::e1_ap_constants::{
        ID_ADDITIONAL_PDCPDUPLICATION_INFORMATION, ID_ALTERNATIVE_QO_SPARA_SET_LIST,
        ID_BCBEARER_CONTEXT_NGU_TNLINFOAT_NGRAN_REQUEST, ID_BROADCAST_F1_U_CONTEXT_REFERENCE_E1,
        ID_CAUSE, ID_CNPACKET_DELAY_BUDGET_DOWNLINK, ID_CNPACKET_DELAY_BUDGET_UPLINK,
        ID_COMMON_NETWORK_INSTANCE, ID_DAPSREQUEST_INFO,
        ID_DATA_FORWARDINGTO_E_UTRANINFORMATION_LIST,
        ID_DATA_FORWARDINGTO_NG_RANQO_SFLOW_INFORMATION_LIST, ID_DATA_FORWARDING_SOURCE_IPADDRESS,
        ID_DISCARD_TIMER_EXTENDED, ID_DRB_QO_S, ID_EARLY_DATA_FORWARDING_INDICATOR,
        ID_EARLY_FORWARDING_COUNTINFO, ID_EARLY_FORWARDING_COUNTREQ,
        ID_ECNMARKINGOR_CONGESTION_INFORMATION_REPORTING_REQUEST,
        ID_ECNMARKINGOR_CONGESTION_INFORMATION_REPORTING_STATUS, ID_EHC_PARAMETERS,
        ID_ENDPOINT_IP_ADDRESS_AND_PORT, ID_EXTENDED_PACKET_DELAY_BUDGET,
        ID_F1_UTUNNEL_NOT_ESTABLISHED, ID_F1_U_TNL_INFO_ADDED_LIST,
        ID_F1_U_TNL_INFO_ADDED_OR_MODIFIED_LIST, ID_F1_U_TNL_INFO_TO_ADD_LIST,
        ID_F1_U_TNL_INFO_TO_ADD_OR_MODIFY_LIST, ID_F1_U_TNL_INFO_TO_RELEASE_LIST,
        ID_IGNORE_MAPPING_RULE_INDICATION, ID_INDIRECT_PATH_INDICATION, ID_M4_REPORT_AMOUNT,
        ID_M6_REPORT_AMOUNT, ID_M7_REPORT_AMOUNT, ID_MAXIMUM_DATA_BURST_VOLUME, ID_MAX_CIDEHCDL,
        ID_MBSAREA_SESSION_ID, ID_MBSSESSION_ASSOCIATED_INFO_NON_SUPPORT_TO_SUPPORT,
        ID_MBSSESSION_RESOURCE_NOTIFICATION, ID_MCBEARER_CONTEXT_INACTIVITY_TIMER,
        ID_MCBEARER_CONTEXT_STATUS_CHANGE, ID_MCFORWARDING_RESOURCE_INDICATION,
        ID_MCFORWARDING_RESOURCE_RELEASE, ID_MCFORWARDING_RESOURCE_RELEASE_INDICATION,
        ID_MCFORWARDING_RESOURCE_REQUEST, ID_MCFORWARDING_RESOURCE_RESPONSE,
        ID_MCG_OFFERED_GBRQO_SFLOW_INFO, ID_MDTCONFIGURATION, ID_N6_JITTER_INFORMATION,
        ID_NETWORK_INSTANCE, ID_NUMBER_OF_TUNNELS, ID_OLD_QO_SFLOW_MAP_ULENDMARKEREXPECTED,
        ID_PDCPSNGAP_REPORT, ID_PDCP_COUNT_RESET, ID_PDCP_STATUS_REPORT_INDICATION,
        ID_PDUSESSION_PAIR_ID, ID_PDUSETBASED_HANDLING_INDICATOR, ID_PDUSET_QO_SPARAMETERS,
        ID_PSIBASED_DISCARD_TIMER, ID_QOS_MONITORING_REPORTING_FREQUENCY,
        ID_QO_SFLOWS_DRBREMAPPING, ID_QO_SFLOW_MAPPING_INDICATION, ID_QO_SMONITORING_DISABLED,
        ID_QO_SMONITORING_REQUEST, ID_QO_S_MAPPING_INFORMATION,
        ID_REDUNDANT_COMMON_NETWORK_INSTANCE, ID_REDUNDANT_N_G_DL_UP_TNL_INFORMATION,
        ID_REDUNDANT_N_G_UL_UP_TNL_INFORMATION, ID_REDUNDANT_PDUSESSION_INFORMATION,
        ID_REDUNDANT_PDUSESSION_INFORMATION_USED, ID_REDUNDANT_QOS_FLOW_INDICATOR,
        ID_SDTINDICATOR_MOD, ID_SDTINDICATOR_SETUP,
        ID_SECONDARY_PDU_SESSION_DATA_FORWARDING_INFORMATION, ID_SECURITY_INDICATION,
        ID_SECURITY_INDICATION_MODIFY, ID_SECURITY_RESULT, ID_SNSSAI,
        ID_SPECIAL_TRIGGERING_PURPOSE, ID_SURVIVAL_TIME,
        ID_TNLASSOCIATION_TRANSPORT_LAYER_ADDRESSG_NBCUUP, ID_TRACE_COLLECTION_ENTITY_URI,
        ID_TSCTRAFFIC_CHARACTERISTICS, ID_UDC_PARAMETERS, ID_USER_PLANE_ERROR_INDICATOR,
        ID_USER_PLANE_FAILURE_INDICATION, ID_VERSION_ID, MAXNOOFTIMEPERIODS, MAXNOOF_CELLSFOR_MBS,
        MAXNOOF_CELL_GROUPS, MAXNOOF_DATA_FORWARDING_TUNNELTO_E_UTRAN, MAXNOOF_DRBS, MAXNOOF_DUS,
        MAXNOOF_ECGI, MAXNOOF_ERRORS, MAXNOOF_EUTRANQOSPARAMETERS, MAXNOOF_EXT_NRCGI,
        MAXNOOF_EXT_SLICE_ITEMS, MAXNOOF_GTPTLAS, MAXNOOF_MBSAREA_SESSION_IDS,
        MAXNOOF_MBSSERVICE_AREA_INFORMATION, MAXNOOF_MBSSESSION_IDS, MAXNOOF_MDTPLMNS,
        MAXNOOF_MRBS, MAXNOOF_NGRANQOSPARAMETERS, MAXNOOF_NRCGI, MAXNOOF_PDUSESSION_RESOURCE,
        MAXNOOF_QO_SFLOWS, MAXNOOF_QO_SPARA_SETS, MAXNOOF_SHARED_NG_UTERMINATIONS,
        MAXNOOF_SLICE_ITEMS, MAXNOOF_SMBRVALUES, MAXNOOF_SPLMNS, MAXNOOF_TAIFOR_MBS, MAXNOOF_TLAS,
        MAXNOOF_UPPARAMETERS,
    };
    use super::e1_ap_containers::{
        ProtocolExtensionContainer, ProtocolIESingleContainer, E1APPROTOCOLEXTENSION,
        E1APPROTOCOLIES,
    };
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum ActivityInformationChoiceExtensionCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ActivityInformationChoiceExtension {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: ActivityInformationChoiceExtensionCriticality,
        pub value: Any,
    }
    impl ActivityInformationChoiceExtension {
        pub fn new(
            id: u16,
            criticality: ActivityInformationChoiceExtensionCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " A"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum ActivityInformation {
        #[rasn(identifier = "dRB-Activity-List")]
        dRB_Activity_List(DRBActivityList),
        #[rasn(identifier = "pDU-Session-Resource-Activity-List")]
        pDU_Session_Resource_Activity_List(PDUSessionResourceActivityList),
        #[rasn(identifier = "uE-Activity")]
        uE_Activity(UEActivity),
        #[rasn(identifier = "choice-extension")]
        choice_extension(ActivityInformationChoiceExtension),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ActivityNotificationLevel {
        drb = 0,
        #[rasn(identifier = "pdu-session")]
        pdu_session = 1,
        ue = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum AdditionalHandoverInfo {
        #[rasn(identifier = "discard-pdpc-SN")]
        discard_pdpc_SN = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum AdditionalPDCPduplicationInformation {
        three = 0,
        four = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct AdditionalRRMPriorityIndex(pub FixedBitString<32usize>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousAlternativeQoSParaSetItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousAlternativeQoSParaSetItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousAlternativeQoSParaSetItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousAlternativeQoSParaSetItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousAlternativeQoSParaSetItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct AlternativeQoSParaSetItemIEExtensions(
        pub SequenceOf<AnonymousAlternativeQoSParaSetItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct AlternativeQoSParaSetItem {
        #[rasn(
            value("1..=8", extensible),
            identifier = "alternativeQoSParameterIndex"
        )]
        pub alternative_qo_sparameter_index: Integer,
        #[rasn(identifier = "guaranteedFlowBitRateDL")]
        pub guaranteed_flow_bit_rate_dl: Option<BitRate>,
        #[rasn(identifier = "guaranteedFlowBitRateUL")]
        pub guaranteed_flow_bit_rate_ul: Option<BitRate>,
        #[rasn(identifier = "packetDelayBudget")]
        pub packet_delay_budget: Option<PacketDelayBudget>,
        #[rasn(identifier = "packetErrorRate")]
        pub packet_error_rate: Option<PacketErrorRate>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<AlternativeQoSParaSetItemIEExtensions>,
    }
    impl AlternativeQoSParaSetItem {
        pub fn new(
            alternative_qo_sparameter_index: Integer,
            guaranteed_flow_bit_rate_dl: Option<BitRate>,
            guaranteed_flow_bit_rate_ul: Option<BitRate>,
            packet_delay_budget: Option<PacketDelayBudget>,
            packet_error_rate: Option<PacketErrorRate>,
            i_e_extensions: Option<AlternativeQoSParaSetItemIEExtensions>,
        ) -> Self {
            Self {
                alternative_qo_sparameter_index,
                guaranteed_flow_bit_rate_dl,
                guaranteed_flow_bit_rate_ul,
                packet_delay_budget,
                packet_error_rate,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8"))]
    pub struct AlternativeQoSParaSetList(pub SequenceOf<AlternativeQoSParaSetItem>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct AssociatedSessionID(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=4095", extensible))]
    pub struct AveragingWindow(pub Integer);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum BCBearerContextF1UTNLInfoatCUChoiceExtensionCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct BCBearerContextF1UTNLInfoatCUChoiceExtension {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: BCBearerContextF1UTNLInfoatCUChoiceExtensionCriticality,
        pub value: Any,
    }
    impl BCBearerContextF1UTNLInfoatCUChoiceExtension {
        pub fn new(
            id: u16,
            criticality: BCBearerContextF1UTNLInfoatCUChoiceExtensionCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags, identifier = "BCBearerContextF1U-TNLInfoatCU")]
    pub enum BCBearerContextF1UTNLInfoatCU {
        locationindependent(MBSF1UInformationAtCU),
        locationdependent(LocationDependentMBSF1UInformationAtCU),
        #[rasn(identifier = "choice-extension")]
        choice_extension(BCBearerContextF1UTNLInfoatCUChoiceExtension),
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum BCBearerContextF1UTNLInfoatDUChoiceExtensionCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct BCBearerContextF1UTNLInfoatDUChoiceExtension {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: BCBearerContextF1UTNLInfoatDUChoiceExtensionCriticality,
        pub value: Any,
    }
    impl BCBearerContextF1UTNLInfoatDUChoiceExtension {
        pub fn new(
            id: u16,
            criticality: BCBearerContextF1UTNLInfoatDUChoiceExtensionCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags, identifier = "BCBearerContextF1U-TNLInfoatDU")]
    pub enum BCBearerContextF1UTNLInfoatDU {
        locationindependent(MBSF1UInformationAtDU),
        locationdependent(LocationDependentMBSF1UInformationAtDU),
        #[rasn(identifier = "choice-extension")]
        choice_extension(BCBearerContextF1UTNLInfoatDUChoiceExtension),
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum BCBearerContextNGUTNLInfoat5GCChoiceExtensionCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct BCBearerContextNGUTNLInfoat5GCChoiceExtension {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: BCBearerContextNGUTNLInfoat5GCChoiceExtensionCriticality,
        pub value: Any,
    }
    impl BCBearerContextNGUTNLInfoat5GCChoiceExtension {
        pub fn new(
            id: u16,
            criticality: BCBearerContextNGUTNLInfoat5GCChoiceExtensionCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags, identifier = "BCBearerContextNGU-TNLInfoat5GC")]
    pub enum BCBearerContextNGUTNLInfoat5GC {
        locationindependent(MBSNGUInformationAt5GC),
        locationdependent(LocationDependentMBSNGUInformationAt5GC),
        #[rasn(identifier = "choice-extension")]
        choice_extension(BCBearerContextNGUTNLInfoat5GCChoiceExtension),
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum BCBearerContextNGUTNLInfoatNGRANChoiceExtensionCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct BCBearerContextNGUTNLInfoatNGRANChoiceExtension {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: BCBearerContextNGUTNLInfoatNGRANChoiceExtensionCriticality,
        pub value: Any,
    }
    impl BCBearerContextNGUTNLInfoatNGRANChoiceExtension {
        pub fn new(
            id: u16,
            criticality: BCBearerContextNGUTNLInfoatNGRANChoiceExtensionCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        choice,
        automatic_tags,
        identifier = "BCBearerContextNGU-TNLInfoatNGRAN"
    )]
    pub enum BCBearerContextNGUTNLInfoatNGRAN {
        locationindependent(MBSNGUInformationAtNGRAN),
        locationdependent(LocationDependentMBSNGUInformationAtNGRAN),
        #[rasn(identifier = "choice-extension")]
        choice_extension(BCBearerContextNGUTNLInfoatNGRANChoiceExtension),
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum BCBearerContextNGUTNLInfoatNGRANRequestChoiceExtensionCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct BCBearerContextNGUTNLInfoatNGRANRequestChoiceExtension {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: BCBearerContextNGUTNLInfoatNGRANRequestChoiceExtensionCriticality,
        pub value: Any,
    }
    impl BCBearerContextNGUTNLInfoatNGRANRequestChoiceExtension {
        pub fn new(
            id: u16,
            criticality: BCBearerContextNGUTNLInfoatNGRANRequestChoiceExtensionCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        choice,
        automatic_tags,
        identifier = "BCBearerContextNGU-TNLInfoatNGRAN-Request"
    )]
    pub enum BCBearerContextNGUTNLInfoatNGRANRequest {
        locationindependent(MBSNGUInformationAtNGRANRequest),
        locationdependent(MBSNGUInformationAtNGRANRequestList),
        #[rasn(identifier = "choice-extension")]
        choice_extension(BCBearerContextNGUTNLInfoatNGRANRequestChoiceExtension),
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBCBearerContextToModifyIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBCBearerContextToModifyIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBCBearerContextToModifyIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousBCBearerContextToModifyIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousBCBearerContextToModifyIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct BCBearerContextToModifyIEExtensions(
        pub SequenceOf<AnonymousBCBearerContextToModifyIEExtensions>,
    );
    #[doc = " BCBearerContextToModify"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BCBearerContextToModify {
        #[rasn(identifier = "bcBearerContextNGU-TNLInfoat5GC")]
        pub bc_bearer_context_ngu_tnlinfoat5_gc: Option<BCBearerContextNGUTNLInfoat5GC>,
        #[rasn(identifier = "bcMRBToSetupList")]
        pub bc_mrbto_setup_list: Option<BCMRBSetupConfiguration>,
        #[rasn(identifier = "bcMRBToModifyList")]
        pub bc_mrbto_modify_list: Option<BCMRBModifyConfiguration>,
        #[rasn(identifier = "bcMRBToRemoveList")]
        pub bc_mrbto_remove_list: Option<BCMRBRemoveConfiguration>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<BCBearerContextToModifyIEExtensions>,
    }
    impl BCBearerContextToModify {
        pub fn new(
            bc_bearer_context_ngu_tnlinfoat5_gc: Option<BCBearerContextNGUTNLInfoat5GC>,
            bc_mrbto_setup_list: Option<BCMRBSetupConfiguration>,
            bc_mrbto_modify_list: Option<BCMRBModifyConfiguration>,
            bc_mrbto_remove_list: Option<BCMRBRemoveConfiguration>,
            i_e_extensions: Option<BCBearerContextToModifyIEExtensions>,
        ) -> Self {
            Self {
                bc_bearer_context_ngu_tnlinfoat5_gc,
                bc_mrbto_setup_list,
                bc_mrbto_modify_list,
                bc_mrbto_remove_list,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBCBearerContextToModifyConfirmIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBCBearerContextToModifyConfirmIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBCBearerContextToModifyConfirmIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousBCBearerContextToModifyConfirmIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousBCBearerContextToModifyConfirmIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct BCBearerContextToModifyConfirmIEExtensions(
        pub SequenceOf<AnonymousBCBearerContextToModifyConfirmIEExtensions>,
    );
    #[doc = " BCBearerContextToModifyConfirm"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BCBearerContextToModifyConfirm {
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<BCBearerContextToModifyConfirmIEExtensions>,
    }
    impl BCBearerContextToModifyConfirm {
        pub fn new(i_e_extensions: Option<BCBearerContextToModifyConfirmIEExtensions>) -> Self {
            Self { i_e_extensions }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBCBearerContextToModifyRequiredIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBCBearerContextToModifyRequiredIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBCBearerContextToModifyRequiredIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousBCBearerContextToModifyRequiredIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousBCBearerContextToModifyRequiredIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct BCBearerContextToModifyRequiredIEExtensions(
        pub SequenceOf<AnonymousBCBearerContextToModifyRequiredIEExtensions>,
    );
    #[doc = " BCBearerContextToModifyRequired"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BCBearerContextToModifyRequired {
        #[rasn(identifier = "bcMRBToRemoveList")]
        pub bc_mrbto_remove_list: Option<BCMRBRemoveConfiguration>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<BCBearerContextToModifyRequiredIEExtensions>,
    }
    impl BCBearerContextToModifyRequired {
        pub fn new(
            bc_mrbto_remove_list: Option<BCMRBRemoveConfiguration>,
            i_e_extensions: Option<BCBearerContextToModifyRequiredIEExtensions>,
        ) -> Self {
            Self {
                bc_mrbto_remove_list,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBCBearerContextToModifyResponseIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBCBearerContextToModifyResponseIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBCBearerContextToModifyResponseIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousBCBearerContextToModifyResponseIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousBCBearerContextToModifyResponseIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct BCBearerContextToModifyResponseIEExtensions(
        pub SequenceOf<AnonymousBCBearerContextToModifyResponseIEExtensions>,
    );
    #[doc = " BCBearerContextToModifyResponse"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BCBearerContextToModifyResponse {
        #[rasn(identifier = "bcBearerContextNGU-TNLInfoatNGRAN")]
        pub bc_bearer_context_ngu_tnlinfoat_ngran: Option<BCBearerContextNGUTNLInfoatNGRAN>,
        #[rasn(identifier = "bcMRBSetupModifyResponseList")]
        pub bc_mrbsetup_modify_response_list: BCMRBSetupModifyResponseList,
        #[rasn(identifier = "bcMRBFailedList")]
        pub bc_mrbfailed_list: Option<BCMRBFailedList>,
        #[rasn(identifier = "availableBCMRBConfig")]
        pub available_bcmrbconfig: Option<BCMRBSetupConfiguration>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<BCBearerContextToModifyResponseIEExtensions>,
    }
    impl BCBearerContextToModifyResponse {
        pub fn new(
            bc_bearer_context_ngu_tnlinfoat_ngran: Option<BCBearerContextNGUTNLInfoatNGRAN>,
            bc_mrbsetup_modify_response_list: BCMRBSetupModifyResponseList,
            bc_mrbfailed_list: Option<BCMRBFailedList>,
            available_bcmrbconfig: Option<BCMRBSetupConfiguration>,
            i_e_extensions: Option<BCBearerContextToModifyResponseIEExtensions>,
        ) -> Self {
            Self {
                bc_bearer_context_ngu_tnlinfoat_ngran,
                bc_mrbsetup_modify_response_list,
                bc_mrbfailed_list,
                available_bcmrbconfig,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBCBearerContextToSetupIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBCBearerContextToSetupIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBCBearerContextToSetupIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousBCBearerContextToSetupIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousBCBearerContextToSetupIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct BCBearerContextToSetupIEExtensions(
        pub SequenceOf<AnonymousBCBearerContextToSetupIEExtensions>,
    );
    #[doc = " B"]
    #[doc = " BCBearerContextToSetup"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BCBearerContextToSetup {
        pub snssai: SNSSAI,
        #[rasn(identifier = "bcBearerContextNGU-TNLInfoat5GC")]
        pub bc_bearer_context_ngu_tnlinfoat5_gc: Option<BCBearerContextNGUTNLInfoat5GC>,
        #[rasn(identifier = "bcMRBToSetupList")]
        pub bc_mrbto_setup_list: BCMRBSetupConfiguration,
        #[rasn(identifier = "requestedAction")]
        pub requested_action: Option<RequestedAction4AvailNGUTermination>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<BCBearerContextToSetupIEExtensions>,
    }
    impl BCBearerContextToSetup {
        pub fn new(
            snssai: SNSSAI,
            bc_bearer_context_ngu_tnlinfoat5_gc: Option<BCBearerContextNGUTNLInfoat5GC>,
            bc_mrbto_setup_list: BCMRBSetupConfiguration,
            requested_action: Option<RequestedAction4AvailNGUTermination>,
            i_e_extensions: Option<BCBearerContextToSetupIEExtensions>,
        ) -> Self {
            Self {
                snssai,
                bc_bearer_context_ngu_tnlinfoat5_gc,
                bc_mrbto_setup_list,
                requested_action,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBCBearerContextToSetupResponseIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBCBearerContextToSetupResponseIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBCBearerContextToSetupResponseIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousBCBearerContextToSetupResponseIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousBCBearerContextToSetupResponseIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct BCBearerContextToSetupResponseIEExtensions(
        pub SequenceOf<AnonymousBCBearerContextToSetupResponseIEExtensions>,
    );
    #[doc = " BCBearerContextToSetupResponse"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BCBearerContextToSetupResponse {
        #[rasn(identifier = "bcBearerContextNGU-TNLInfoatNGRAN")]
        pub bc_bearer_context_ngu_tnlinfoat_ngran: Option<BCBearerContextNGUTNLInfoatNGRAN>,
        #[rasn(identifier = "bcMRBSetupResponseList")]
        pub bc_mrbsetup_response_list: BCMRBSetupResponseList,
        #[rasn(identifier = "bcMRBFailedList")]
        pub bc_mrbfailed_list: Option<BCMRBFailedList>,
        #[rasn(identifier = "availableBCMRBConfig")]
        pub available_bcmrbconfig: Option<BCMRBSetupConfiguration>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<BCBearerContextToSetupResponseIEExtensions>,
    }
    impl BCBearerContextToSetupResponse {
        pub fn new(
            bc_bearer_context_ngu_tnlinfoat_ngran: Option<BCBearerContextNGUTNLInfoatNGRAN>,
            bc_mrbsetup_response_list: BCMRBSetupResponseList,
            bc_mrbfailed_list: Option<BCMRBFailedList>,
            available_bcmrbconfig: Option<BCMRBSetupConfiguration>,
            i_e_extensions: Option<BCBearerContextToSetupResponseIEExtensions>,
        ) -> Self {
            Self {
                bc_bearer_context_ngu_tnlinfoat_ngran,
                bc_mrbsetup_response_list,
                bc_mrbfailed_list,
                available_bcmrbconfig,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct BCMRBFailedList(pub SequenceOf<BCMRBFailedListItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBCMRBFailedListItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBCMRBFailedListItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBCMRBFailedListItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousBCMRBFailedListItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousBCMRBFailedListItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct BCMRBFailedListItemIEExtensions(
        pub SequenceOf<AnonymousBCMRBFailedListItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "BCMRBFailedList-Item")]
    #[non_exhaustive]
    pub struct BCMRBFailedListItem {
        #[rasn(identifier = "mrb-ID")]
        pub mrb_id: MRBID,
        pub cause: Cause,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<BCMRBFailedListItemIEExtensions>,
    }
    impl BCMRBFailedListItem {
        pub fn new(
            mrb_id: MRBID,
            cause: Cause,
            i_e_extensions: Option<BCMRBFailedListItemIEExtensions>,
        ) -> Self {
            Self {
                mrb_id,
                cause,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct BCMRBModifyConfiguration(pub SequenceOf<BCMRBModifyConfigurationItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBCMRBModifyConfigurationItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBCMRBModifyConfigurationItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBCMRBModifyConfigurationItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousBCMRBModifyConfigurationItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousBCMRBModifyConfigurationItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct BCMRBModifyConfigurationItemIEExtensions(
        pub SequenceOf<AnonymousBCMRBModifyConfigurationItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "BCMRBModifyConfiguration-Item")]
    #[non_exhaustive]
    pub struct BCMRBModifyConfigurationItem {
        #[rasn(identifier = "mrb-ID")]
        pub mrb_id: MRBID,
        #[rasn(identifier = "bcBearerContextF1U-TNLInfoatDU")]
        pub bc_bearer_context_f1_u_tnlinfoat_du: Option<BCBearerContextF1UTNLInfoatDU>,
        #[rasn(identifier = "mbs-pdcp-config")]
        pub mbs_pdcp_config: Option<PDCPConfiguration>,
        #[rasn(identifier = "qoS-Flow-QoS-Parameter-List")]
        pub qo_s_flow_qo_s_parameter_list: Option<QoSFlowQoSParameterList>,
        #[rasn(identifier = "qoSFlowLevelQoSParameters")]
        pub qo_sflow_level_qo_sparameters: Option<QoSFlowLevelQoSParameters>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<BCMRBModifyConfigurationItemIEExtensions>,
    }
    impl BCMRBModifyConfigurationItem {
        pub fn new(
            mrb_id: MRBID,
            bc_bearer_context_f1_u_tnlinfoat_du: Option<BCBearerContextF1UTNLInfoatDU>,
            mbs_pdcp_config: Option<PDCPConfiguration>,
            qo_s_flow_qo_s_parameter_list: Option<QoSFlowQoSParameterList>,
            qo_sflow_level_qo_sparameters: Option<QoSFlowLevelQoSParameters>,
            i_e_extensions: Option<BCMRBModifyConfigurationItemIEExtensions>,
        ) -> Self {
            Self {
                mrb_id,
                bc_bearer_context_f1_u_tnlinfoat_du,
                mbs_pdcp_config,
                qo_s_flow_qo_s_parameter_list,
                qo_sflow_level_qo_sparameters,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct BCMRBRemoveConfiguration(pub SequenceOf<MRBID>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct BCMRBSetupConfiguration(pub SequenceOf<BCMRBSetupConfigurationItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBCMRBSetupConfigurationItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBCMRBSetupConfigurationItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBCMRBSetupConfigurationItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousBCMRBSetupConfigurationItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousBCMRBSetupConfigurationItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct BCMRBSetupConfigurationItemIEExtensions(
        pub SequenceOf<AnonymousBCMRBSetupConfigurationItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "BCMRBSetupConfiguration-Item")]
    #[non_exhaustive]
    pub struct BCMRBSetupConfigurationItem {
        #[rasn(identifier = "mrb-ID")]
        pub mrb_id: MRBID,
        #[rasn(identifier = "mbs-pdcp-config")]
        pub mbs_pdcp_config: PDCPConfiguration,
        #[rasn(identifier = "qoS-Flow-QoS-Parameter-List")]
        pub qo_s_flow_qo_s_parameter_list: QoSFlowQoSParameterList,
        #[rasn(identifier = "qoSFlowLevelQoSParameters")]
        pub qo_sflow_level_qo_sparameters: Option<QoSFlowLevelQoSParameters>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<BCMRBSetupConfigurationItemIEExtensions>,
    }
    impl BCMRBSetupConfigurationItem {
        pub fn new(
            mrb_id: MRBID,
            mbs_pdcp_config: PDCPConfiguration,
            qo_s_flow_qo_s_parameter_list: QoSFlowQoSParameterList,
            qo_sflow_level_qo_sparameters: Option<QoSFlowLevelQoSParameters>,
            i_e_extensions: Option<BCMRBSetupConfigurationItemIEExtensions>,
        ) -> Self {
            Self {
                mrb_id,
                mbs_pdcp_config,
                qo_s_flow_qo_s_parameter_list,
                qo_sflow_level_qo_sparameters,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct BCMRBSetupModifyResponseList(pub SequenceOf<BCMRBSetupModifyResponseListItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBCMRBSetupModifyResponseListItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBCMRBSetupModifyResponseListItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBCMRBSetupModifyResponseListItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousBCMRBSetupModifyResponseListItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousBCMRBSetupModifyResponseListItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct BCMRBSetupModifyResponseListItemIEExtensions(
        pub SequenceOf<AnonymousBCMRBSetupModifyResponseListItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "BCMRBSetupModifyResponseList-Item")]
    #[non_exhaustive]
    pub struct BCMRBSetupModifyResponseListItem {
        #[rasn(identifier = "mrb-ID")]
        pub mrb_id: MRBID,
        #[rasn(identifier = "qosflow-setup")]
        pub qosflow_setup: Option<QoSFlowList>,
        #[rasn(identifier = "qosflow-failed")]
        pub qosflow_failed: Option<QoSFlowFailedList>,
        #[rasn(identifier = "bcBearerContextF1U-TNLInfoatCU")]
        pub bc_bearer_context_f1_u_tnlinfoat_cu: Option<BCBearerContextF1UTNLInfoatCU>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<BCMRBSetupModifyResponseListItemIEExtensions>,
    }
    impl BCMRBSetupModifyResponseListItem {
        pub fn new(
            mrb_id: MRBID,
            qosflow_setup: Option<QoSFlowList>,
            qosflow_failed: Option<QoSFlowFailedList>,
            bc_bearer_context_f1_u_tnlinfoat_cu: Option<BCBearerContextF1UTNLInfoatCU>,
            i_e_extensions: Option<BCMRBSetupModifyResponseListItemIEExtensions>,
        ) -> Self {
            Self {
                mrb_id,
                qosflow_setup,
                qosflow_failed,
                bc_bearer_context_f1_u_tnlinfoat_cu,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct BCMRBSetupResponseList(pub SequenceOf<BCMRBSetupResponseListItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBCMRBSetupResponseListItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBCMRBSetupResponseListItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBCMRBSetupResponseListItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousBCMRBSetupResponseListItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousBCMRBSetupResponseListItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct BCMRBSetupResponseListItemIEExtensions(
        pub SequenceOf<AnonymousBCMRBSetupResponseListItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "BCMRBSetupResponseList-Item")]
    #[non_exhaustive]
    pub struct BCMRBSetupResponseListItem {
        #[rasn(identifier = "mrb-ID")]
        pub mrb_id: MRBID,
        #[rasn(identifier = "qosflow-setup")]
        pub qosflow_setup: QoSFlowList,
        #[rasn(identifier = "qosflow-failed")]
        pub qosflow_failed: Option<QoSFlowFailedList>,
        #[rasn(identifier = "bcBearerContextF1U-TNLInfoatCU")]
        pub bc_bearer_context_f1_u_tnlinfoat_cu: BCBearerContextF1UTNLInfoatCU,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<BCMRBSetupResponseListItemIEExtensions>,
    }
    impl BCMRBSetupResponseListItem {
        pub fn new(
            mrb_id: MRBID,
            qosflow_setup: QoSFlowList,
            qosflow_failed: Option<QoSFlowFailedList>,
            bc_bearer_context_f1_u_tnlinfoat_cu: BCBearerContextF1UTNLInfoatCU,
            i_e_extensions: Option<BCMRBSetupResponseListItemIEExtensions>,
        ) -> Self {
            Self {
                mrb_id,
                qosflow_setup,
                qosflow_failed,
                bc_bearer_context_f1_u_tnlinfoat_cu,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum BearerContextStatusChange {
        suspend = 0,
        resume = 1,
        #[rasn(extension_addition)]
        resumeforSDT = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=4000000000000", extensible))]
    pub struct BitRate(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "BroadcastF1U-ContextReferenceE1")]
    pub struct BroadcastF1UContextReferenceE1(pub FixedOctetString<4usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum BufferSize {
        kbyte2 = 0,
        kbyte4 = 1,
        kbyte8 = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct BurstArrivalTime(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum CHOInitiation {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum CNSupport {
        #[rasn(identifier = "c-epc")]
        c_epc = 0,
        #[rasn(identifier = "c-5gc")]
        c_5gc = 1,
        both = 2,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum CPTNLInformationChoiceExtensionCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct CPTNLInformationChoiceExtension {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: CPTNLInformationChoiceExtensionCriticality,
        pub value: Any,
    }
    impl CPTNLInformationChoiceExtension {
        pub fn new(
            id: u16,
            criticality: CPTNLInformationChoiceExtensionCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags, identifier = "CP-TNL-Information")]
    pub enum CPTNLInformation {
        #[rasn(identifier = "endpoint-IP-Address")]
        endpoint_IP_Address(TransportLayerAddress),
        #[rasn(identifier = "choice-extension")]
        choice_extension(CPTNLInformationChoiceExtension),
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum CauseChoiceExtensionCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct CauseChoiceExtension {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: CauseChoiceExtensionCriticality,
        pub value: Any,
    }
    impl CauseChoiceExtension {
        pub fn new(id: u16, criticality: CauseChoiceExtensionCriticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " C"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum Cause {
        radioNetwork(CauseRadioNetwork),
        transport(CauseTransport),
        protocol(CauseProtocol),
        misc(CauseMisc),
        #[rasn(identifier = "choice-extension")]
        choice_extension(CauseChoiceExtension),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum CauseMisc {
        #[rasn(identifier = "control-processing-overload")]
        control_processing_overload = 0,
        #[rasn(identifier = "not-enough-user-plane-processing-resources")]
        not_enough_user_plane_processing_resources = 1,
        #[rasn(identifier = "hardware-failure")]
        hardware_failure = 2,
        #[rasn(identifier = "om-intervention")]
        om_intervention = 3,
        unspecified = 4,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum CauseProtocol {
        #[rasn(identifier = "transfer-syntax-error")]
        transfer_syntax_error = 0,
        #[rasn(identifier = "abstract-syntax-error-reject")]
        abstract_syntax_error_reject = 1,
        #[rasn(identifier = "abstract-syntax-error-ignore-and-notify")]
        abstract_syntax_error_ignore_and_notify = 2,
        #[rasn(identifier = "message-not-compatible-with-receiver-state")]
        message_not_compatible_with_receiver_state = 3,
        #[rasn(identifier = "semantic-error")]
        semantic_error = 4,
        #[rasn(identifier = "abstract-syntax-error-falsely-constructed-message")]
        abstract_syntax_error_falsely_constructed_message = 5,
        unspecified = 6,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum CauseRadioNetwork {
        unspecified = 0,
        #[rasn(identifier = "unknown-or-already-allocated-gnb-cu-cp-ue-e1ap-id")]
        unknown_or_already_allocated_gnb_cu_cp_ue_e1ap_id = 1,
        #[rasn(identifier = "unknown-or-already-allocated-gnb-cu-up-ue-e1ap-id")]
        unknown_or_already_allocated_gnb_cu_up_ue_e1ap_id = 2,
        #[rasn(identifier = "unknown-or-inconsistent-pair-of-ue-e1ap-id")]
        unknown_or_inconsistent_pair_of_ue_e1ap_id = 3,
        #[rasn(identifier = "interaction-with-other-procedure")]
        interaction_with_other_procedure = 4,
        #[rasn(identifier = "pPDCP-Count-wrap-around")]
        pPDCP_Count_wrap_around = 5,
        #[rasn(identifier = "not-supported-QCI-value")]
        not_supported_QCI_value = 6,
        #[rasn(identifier = "not-supported-5QI-value")]
        not_supported_5QI_value = 7,
        #[rasn(identifier = "encryption-algorithms-not-supported")]
        encryption_algorithms_not_supported = 8,
        #[rasn(identifier = "integrity-protection-algorithms-not-supported")]
        integrity_protection_algorithms_not_supported = 9,
        #[rasn(identifier = "uP-integrity-protection-not-possible")]
        uP_integrity_protection_not_possible = 10,
        #[rasn(identifier = "uP-confidentiality-protection-not-possible")]
        uP_confidentiality_protection_not_possible = 11,
        #[rasn(identifier = "multiple-PDU-Session-ID-Instances")]
        multiple_PDU_Session_ID_Instances = 12,
        #[rasn(identifier = "unknown-PDU-Session-ID")]
        unknown_PDU_Session_ID = 13,
        #[rasn(identifier = "multiple-QoS-Flow-ID-Instances")]
        multiple_QoS_Flow_ID_Instances = 14,
        #[rasn(identifier = "unknown-QoS-Flow-ID")]
        unknown_QoS_Flow_ID = 15,
        #[rasn(identifier = "multiple-DRB-ID-Instances")]
        multiple_DRB_ID_Instances = 16,
        #[rasn(identifier = "unknown-DRB-ID")]
        unknown_DRB_ID = 17,
        #[rasn(identifier = "invalid-QoS-combination")]
        invalid_QoS_combination = 18,
        #[rasn(identifier = "procedure-cancelled")]
        procedure_cancelled = 19,
        #[rasn(identifier = "normal-release")]
        normal_release = 20,
        #[rasn(identifier = "no-radio-resources-available")]
        no_radio_resources_available = 21,
        #[rasn(identifier = "action-desirable-for-radio-reasons")]
        action_desirable_for_radio_reasons = 22,
        #[rasn(identifier = "resources-not-available-for-the-slice")]
        resources_not_available_for_the_slice = 23,
        #[rasn(identifier = "pDCP-configuration-not-supported")]
        pDCP_configuration_not_supported = 24,
        #[rasn(extension_addition, identifier = "ue-dl-max-IP-data-rate-reason")]
        ue_dl_max_IP_data_rate_reason = 25,
        #[rasn(extension_addition, identifier = "uP-integrity-protection-failure")]
        uP_integrity_protection_failure = 26,
        #[rasn(extension_addition, identifier = "release-due-to-pre-emption")]
        release_due_to_pre_emption = 27,
        #[rasn(extension_addition, identifier = "rsn-not-available-for-the-up")]
        rsn_not_available_for_the_up = 28,
        #[rasn(extension_addition, identifier = "nPN-not-supported")]
        nPN_not_supported = 29,
        #[rasn(extension_addition, identifier = "report-characteristic-empty")]
        report_characteristic_empty = 30,
        #[rasn(extension_addition, identifier = "existing-measurement-ID")]
        existing_measurement_ID = 31,
        #[rasn(
            extension_addition,
            identifier = "measurement-temporarily-not-available"
        )]
        measurement_temporarily_not_available = 32,
        #[rasn(
            extension_addition,
            identifier = "measurement-not-supported-for-the-object"
        )]
        measurement_not_supported_for_the_object = 33,
        #[rasn(extension_addition, identifier = "scg-activation-deactivation-failure")]
        scg_activation_deactivation_failure = 34,
        #[rasn(
            extension_addition,
            identifier = "scg-deactivation-failure-due-to-data-transmission"
        )]
        scg_deactivation_failure_due_to_data_transmission = 35,
        #[rasn(
            extension_addition,
            identifier = "unknown-or-already-allocated-gNB-CU-CP-MBS-E1AP-ID"
        )]
        unknown_or_already_allocated_gNB_CU_CP_MBS_E1AP_ID = 36,
        #[rasn(
            extension_addition,
            identifier = "unknown-or-already-allocated-gNB-CU-UP-MBS-E1AP-ID"
        )]
        unknown_or_already_allocated_gNB_CU_UP_MBS_E1AP_ID = 37,
        #[rasn(
            extension_addition,
            identifier = "unknown-or-inconsistent-pair-of-MBS-E1AP-ID"
        )]
        unknown_or_inconsistent_pair_of_MBS_E1AP_ID = 38,
        #[rasn(extension_addition, identifier = "unknown-or-inconsistent-MRB-ID")]
        unknown_or_inconsistent_MRB_ID = 39,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum CauseTransport {
        unspecified = 0,
        #[rasn(identifier = "transport-resource-unavailable")]
        transport_resource_unavailable = 1,
        #[rasn(extension_addition, identifier = "unknown-TNL-address-for-IAB")]
        unknown_TNL_address_for_IAB = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Cell-Group-ID", value("0..=3", extensible))]
    pub struct CellGroupID(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"), identifier = "Cell-Group-Information")]
    pub struct CellGroupInformation(pub SequenceOf<CellGroupInformationItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousCellGroupInformationItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousCellGroupInformationItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousCellGroupInformationItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousCellGroupInformationItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousCellGroupInformationItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct CellGroupInformationItemIEExtensions(
        pub SequenceOf<AnonymousCellGroupInformationItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Cell-Group-Information-Item")]
    #[non_exhaustive]
    pub struct CellGroupInformationItem {
        #[rasn(identifier = "cell-Group-ID")]
        pub cell_group_id: CellGroupID,
        #[rasn(identifier = "uL-Configuration")]
        pub u_l_configuration: Option<ULConfiguration>,
        #[rasn(identifier = "dL-TX-Stop")]
        pub d_l_tx_stop: Option<DLTXStop>,
        #[rasn(identifier = "rAT-Type")]
        pub r_at_type: Option<RATType>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<CellGroupInformationItemIEExtensions>,
    }
    impl CellGroupInformationItem {
        pub fn new(
            cell_group_id: CellGroupID,
            u_l_configuration: Option<ULConfiguration>,
            d_l_tx_stop: Option<DLTXStop>,
            r_at_type: Option<RATType>,
            i_e_extensions: Option<CellGroupInformationItemIEExtensions>,
        ) -> Self {
            Self {
                cell_group_id,
                u_l_configuration,
                d_l_tx_stop,
                r_at_type,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum CipheringAlgorithm {
        nEA0 = 0,
        #[rasn(identifier = "c-128-NEA1")]
        c_128_NEA1 = 1,
        #[rasn(identifier = "c-128-NEA2")]
        c_128_NEA2 = 2,
        #[rasn(identifier = "c-128-NEA3")]
        c_128_NEA3 = 3,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct CommonNetworkInstance(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ConfidentialityProtectionIndication {
        required = 0,
        preferred = 1,
        #[rasn(identifier = "not-needed")]
        not_needed = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ConfidentialityProtectionResult {
        performed = 0,
        #[rasn(identifier = "not-performed")]
        not_performed = 1,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousCriticalityDiagnosticsIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousCriticalityDiagnosticsIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousCriticalityDiagnosticsIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousCriticalityDiagnosticsIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousCriticalityDiagnosticsIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct CriticalityDiagnosticsIEExtensions(
        pub SequenceOf<AnonymousCriticalityDiagnosticsIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct CriticalityDiagnostics {
        #[rasn(identifier = "procedureCode")]
        pub procedure_code: Option<ProcedureCode>,
        #[rasn(identifier = "triggeringMessage")]
        pub triggering_message: Option<TriggeringMessage>,
        #[rasn(identifier = "procedureCriticality")]
        pub procedure_criticality: Option<Criticality>,
        #[rasn(identifier = "transactionID")]
        pub transaction_id: Option<TransactionID>,
        #[rasn(identifier = "iEsCriticalityDiagnostics")]
        pub i_es_criticality_diagnostics: Option<CriticalityDiagnosticsIEList>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<CriticalityDiagnosticsIEExtensions>,
    }
    impl CriticalityDiagnostics {
        pub fn new(
            procedure_code: Option<ProcedureCode>,
            triggering_message: Option<TriggeringMessage>,
            procedure_criticality: Option<Criticality>,
            transaction_id: Option<TransactionID>,
            i_es_criticality_diagnostics: Option<CriticalityDiagnosticsIEList>,
            i_e_extensions: Option<CriticalityDiagnosticsIEExtensions>,
        ) -> Self {
            Self {
                procedure_code,
                triggering_message,
                procedure_criticality,
                transaction_id,
                i_es_criticality_diagnostics,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousAnonymousCriticalityDiagnosticsIEListIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousAnonymousCriticalityDiagnosticsIEListIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousAnonymousCriticalityDiagnosticsIEListIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousAnonymousCriticalityDiagnosticsIEListIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousAnonymousCriticalityDiagnosticsIEListIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct AnonymousCriticalityDiagnosticsIEListIEExtensions(
        pub SequenceOf<AnonymousAnonymousCriticalityDiagnosticsIEListIEExtensions>,
    );
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    #[non_exhaustive]
    pub struct AnonymousCriticalityDiagnosticsIEList {
        #[rasn(identifier = "iECriticality")]
        pub i_ecriticality: Criticality,
        #[rasn(identifier = "iE-ID")]
        pub i_e_id: ProtocolIEID,
        #[rasn(identifier = "typeOfError")]
        pub type_of_error: TypeOfError,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<AnonymousCriticalityDiagnosticsIEListIEExtensions>,
    }
    impl AnonymousCriticalityDiagnosticsIEList {
        pub fn new(
            i_ecriticality: Criticality,
            i_e_id: ProtocolIEID,
            type_of_error: TypeOfError,
            i_e_extensions: Option<AnonymousCriticalityDiagnosticsIEListIEExtensions>,
        ) -> Self {
            Self {
                i_ecriticality,
                i_e_id,
                type_of_error,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "CriticalityDiagnostics-IE-List"
    )]
    pub struct CriticalityDiagnosticsIEList(pub SequenceOf<AnonymousCriticalityDiagnosticsIEList>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum DAPSRequestInfoDapsIndicator {
        #[rasn(identifier = "daps-HO-required")]
        daps_HO_required = 0,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDAPSRequestInfoIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDAPSRequestInfoIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDAPSRequestInfoIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDAPSRequestInfoIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDAPSRequestInfoIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DAPSRequestInfoIEExtensions(pub SequenceOf<AnonymousDAPSRequestInfoIEExtensions>);
    #[doc = " D"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct DAPSRequestInfo {
        #[rasn(identifier = "dapsIndicator")]
        pub daps_indicator: DAPSRequestInfoDapsIndicator,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DAPSRequestInfoIEExtensions>,
    }
    impl DAPSRequestInfo {
        pub fn new(
            daps_indicator: DAPSRequestInfoDapsIndicator,
            i_e_extensions: Option<DAPSRequestInfoIEExtensions>,
        ) -> Self {
            Self {
                daps_indicator,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "DL-TX-Stop")]
    #[non_exhaustive]
    pub enum DLTXStop {
        stop = 0,
        resume = 1,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDLDiscardingIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDLDiscardingIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDLDiscardingIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDLDiscardingIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDLDiscardingIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DLDiscardingIEExtensions(pub SequenceOf<AnonymousDLDiscardingIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct DLDiscarding {
        #[rasn(identifier = "dLDiscardingCountVal")]
        pub d_ldiscarding_count_val: PDCPCount,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DLDiscardingIEExtensions>,
    }
    impl DLDiscarding {
        pub fn new(
            d_ldiscarding_count_val: PDCPCount,
            i_e_extensions: Option<DLDiscardingIEExtensions>,
        ) -> Self {
            Self {
                d_ldiscarding_count_val,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDLUPTNLAddressToUpdateItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDLUPTNLAddressToUpdateItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDLUPTNLAddressToUpdateItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDLUPTNLAddressToUpdateItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDLUPTNLAddressToUpdateItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DLUPTNLAddressToUpdateItemIEExtensions(
        pub SequenceOf<AnonymousDLUPTNLAddressToUpdateItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct DLUPTNLAddressToUpdateItem {
        #[rasn(identifier = "oldTNLAdress")]
        pub old_tnladress: TransportLayerAddress,
        #[rasn(identifier = "newTNLAdress")]
        pub new_tnladress: TransportLayerAddress,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DLUPTNLAddressToUpdateItemIEExtensions>,
    }
    impl DLUPTNLAddressToUpdateItem {
        pub fn new(
            old_tnladress: TransportLayerAddress,
            new_tnladress: TransportLayerAddress,
            i_e_extensions: Option<DLUPTNLAddressToUpdateItemIEExtensions>,
        ) -> Self {
            Self {
                old_tnladress,
                new_tnladress,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "DRB-Activity")]
    #[non_exhaustive]
    pub enum DRBActivity {
        active = 0,
        #[rasn(identifier = "not-active")]
        not_active = 1,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBActivityItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBActivityItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBActivityItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBActivityItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBActivityItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBActivityItemIEExtensions(pub SequenceOf<AnonymousDRBActivityItemIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-Activity-Item")]
    #[non_exhaustive]
    pub struct DRBActivityItem {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "dRB-Activity")]
        pub d_rb_activity: DRBActivity,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBActivityItemIEExtensions>,
    }
    impl DRBActivityItem {
        pub fn new(
            d_rb_id: DRBID,
            d_rb_activity: DRBActivity,
            i_e_extensions: Option<DRBActivityItemIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                d_rb_activity,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "DRB-Activity-List")]
    pub struct DRBActivityList(pub SequenceOf<DRBActivityItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBConfirmModifiedItemEUTRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBConfirmModifiedItemEUTRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBConfirmModifiedItemEUTRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBConfirmModifiedItemEUTRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBConfirmModifiedItemEUTRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBConfirmModifiedItemEUTRANIEExtensions(
        pub SequenceOf<AnonymousDRBConfirmModifiedItemEUTRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-Confirm-Modified-Item-EUTRAN")]
    #[non_exhaustive]
    pub struct DRBConfirmModifiedItemEUTRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "cell-Group-Information")]
        pub cell_group_information: Option<CellGroupInformation>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBConfirmModifiedItemEUTRANIEExtensions>,
    }
    impl DRBConfirmModifiedItemEUTRAN {
        pub fn new(
            d_rb_id: DRBID,
            cell_group_information: Option<CellGroupInformation>,
            i_e_extensions: Option<DRBConfirmModifiedItemEUTRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                cell_group_information,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBConfirmModifiedItemNGRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBConfirmModifiedItemNGRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBConfirmModifiedItemNGRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBConfirmModifiedItemNGRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBConfirmModifiedItemNGRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBConfirmModifiedItemNGRANIEExtensions(
        pub SequenceOf<AnonymousDRBConfirmModifiedItemNGRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-Confirm-Modified-Item-NG-RAN")]
    #[non_exhaustive]
    pub struct DRBConfirmModifiedItemNGRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "cell-Group-Information")]
        pub cell_group_information: Option<CellGroupInformation>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBConfirmModifiedItemNGRANIEExtensions>,
    }
    impl DRBConfirmModifiedItemNGRAN {
        pub fn new(
            d_rb_id: DRBID,
            cell_group_information: Option<CellGroupInformation>,
            i_e_extensions: Option<DRBConfirmModifiedItemNGRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                cell_group_information,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=32"),
        identifier = "DRB-Confirm-Modified-List-EUTRAN"
    )]
    pub struct DRBConfirmModifiedListEUTRAN(pub SequenceOf<DRBConfirmModifiedItemEUTRAN>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=32"),
        identifier = "DRB-Confirm-Modified-List-NG-RAN"
    )]
    pub struct DRBConfirmModifiedListNGRAN(pub SequenceOf<DRBConfirmModifiedItemNGRAN>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBFailedItemEUTRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBFailedItemEUTRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBFailedItemEUTRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBFailedItemEUTRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBFailedItemEUTRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBFailedItemEUTRANIEExtensions(
        pub SequenceOf<AnonymousDRBFailedItemEUTRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-Failed-Item-EUTRAN")]
    #[non_exhaustive]
    pub struct DRBFailedItemEUTRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        pub cause: Cause,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBFailedItemEUTRANIEExtensions>,
    }
    impl DRBFailedItemEUTRAN {
        pub fn new(
            d_rb_id: DRBID,
            cause: Cause,
            i_e_extensions: Option<DRBFailedItemEUTRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                cause,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBFailedItemNGRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBFailedItemNGRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBFailedItemNGRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBFailedItemNGRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBFailedItemNGRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBFailedItemNGRANIEExtensions(
        pub SequenceOf<AnonymousDRBFailedItemNGRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-Failed-Item-NG-RAN")]
    #[non_exhaustive]
    pub struct DRBFailedItemNGRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        pub cause: Cause,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBFailedItemNGRANIEExtensions>,
    }
    impl DRBFailedItemNGRAN {
        pub fn new(
            d_rb_id: DRBID,
            cause: Cause,
            i_e_extensions: Option<DRBFailedItemNGRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                cause,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "DRB-Failed-List-EUTRAN")]
    pub struct DRBFailedListEUTRAN(pub SequenceOf<DRBFailedItemEUTRAN>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "DRB-Failed-List-NG-RAN")]
    pub struct DRBFailedListNGRAN(pub SequenceOf<DRBFailedItemNGRAN>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBFailedModItemEUTRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBFailedModItemEUTRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBFailedModItemEUTRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBFailedModItemEUTRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBFailedModItemEUTRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBFailedModItemEUTRANIEExtensions(
        pub SequenceOf<AnonymousDRBFailedModItemEUTRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-Failed-Mod-Item-EUTRAN")]
    #[non_exhaustive]
    pub struct DRBFailedModItemEUTRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        pub cause: Cause,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBFailedModItemEUTRANIEExtensions>,
    }
    impl DRBFailedModItemEUTRAN {
        pub fn new(
            d_rb_id: DRBID,
            cause: Cause,
            i_e_extensions: Option<DRBFailedModItemEUTRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                cause,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBFailedModItemNGRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBFailedModItemNGRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBFailedModItemNGRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBFailedModItemNGRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBFailedModItemNGRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBFailedModItemNGRANIEExtensions(
        pub SequenceOf<AnonymousDRBFailedModItemNGRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-Failed-Mod-Item-NG-RAN")]
    #[non_exhaustive]
    pub struct DRBFailedModItemNGRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        pub cause: Cause,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBFailedModItemNGRANIEExtensions>,
    }
    impl DRBFailedModItemNGRAN {
        pub fn new(
            d_rb_id: DRBID,
            cause: Cause,
            i_e_extensions: Option<DRBFailedModItemNGRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                cause,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "DRB-Failed-Mod-List-EUTRAN")]
    pub struct DRBFailedModListEUTRAN(pub SequenceOf<DRBFailedModItemEUTRAN>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "DRB-Failed-Mod-List-NG-RAN")]
    pub struct DRBFailedModListNGRAN(pub SequenceOf<DRBFailedModItemNGRAN>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBFailedToModifyItemEUTRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBFailedToModifyItemEUTRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBFailedToModifyItemEUTRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBFailedToModifyItemEUTRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBFailedToModifyItemEUTRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBFailedToModifyItemEUTRANIEExtensions(
        pub SequenceOf<AnonymousDRBFailedToModifyItemEUTRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-Failed-To-Modify-Item-EUTRAN")]
    #[non_exhaustive]
    pub struct DRBFailedToModifyItemEUTRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        pub cause: Cause,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBFailedToModifyItemEUTRANIEExtensions>,
    }
    impl DRBFailedToModifyItemEUTRAN {
        pub fn new(
            d_rb_id: DRBID,
            cause: Cause,
            i_e_extensions: Option<DRBFailedToModifyItemEUTRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                cause,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBFailedToModifyItemNGRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBFailedToModifyItemNGRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBFailedToModifyItemNGRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBFailedToModifyItemNGRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBFailedToModifyItemNGRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBFailedToModifyItemNGRANIEExtensions(
        pub SequenceOf<AnonymousDRBFailedToModifyItemNGRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-Failed-To-Modify-Item-NG-RAN")]
    #[non_exhaustive]
    pub struct DRBFailedToModifyItemNGRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        pub cause: Cause,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBFailedToModifyItemNGRANIEExtensions>,
    }
    impl DRBFailedToModifyItemNGRAN {
        pub fn new(
            d_rb_id: DRBID,
            cause: Cause,
            i_e_extensions: Option<DRBFailedToModifyItemNGRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                cause,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=32"),
        identifier = "DRB-Failed-To-Modify-List-EUTRAN"
    )]
    pub struct DRBFailedToModifyListEUTRAN(pub SequenceOf<DRBFailedToModifyItemEUTRAN>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=32"),
        identifier = "DRB-Failed-To-Modify-List-NG-RAN"
    )]
    pub struct DRBFailedToModifyListNGRAN(pub SequenceOf<DRBFailedToModifyItemNGRAN>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "DRB-ID", value("1..=32", extensible))]
    pub struct DRBID(pub Integer);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBMeasurementResultsInformationItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBMeasurementResultsInformationItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBMeasurementResultsInformationItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBMeasurementResultsInformationItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBMeasurementResultsInformationItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBMeasurementResultsInformationItemIEExtensions(
        pub SequenceOf<AnonymousDRBMeasurementResultsInformationItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "DRB-Measurement-Results-Information-Item"
    )]
    #[non_exhaustive]
    pub struct DRBMeasurementResultsInformationItem {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(value("0..=10000", extensible), identifier = "uL-D1-Result")]
        pub u_l_d1_result: Option<Integer>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBMeasurementResultsInformationItemIEExtensions>,
    }
    impl DRBMeasurementResultsInformationItem {
        pub fn new(
            d_rb_id: DRBID,
            u_l_d1_result: Option<Integer>,
            i_e_extensions: Option<DRBMeasurementResultsInformationItemIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                u_l_d1_result,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=32"),
        identifier = "DRB-Measurement-Results-Information-List"
    )]
    pub struct DRBMeasurementResultsInformationList(
        pub SequenceOf<DRBMeasurementResultsInformationItem>,
    );
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBModifiedItemEUTRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBModifiedItemEUTRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBModifiedItemEUTRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBModifiedItemEUTRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBModifiedItemEUTRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBModifiedItemEUTRANIEExtensions(
        pub SequenceOf<AnonymousDRBModifiedItemEUTRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-Modified-Item-EUTRAN")]
    #[non_exhaustive]
    pub struct DRBModifiedItemEUTRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "s1-DL-UP-TNL-Information")]
        pub s1_dl_up_tnl_information: Option<UPTNLInformation>,
        #[rasn(identifier = "pDCP-SN-Status-Information")]
        pub p_dcp_sn_status_information: Option<PDCPSNStatusInformation>,
        #[rasn(identifier = "uL-UP-Transport-Parameters")]
        pub u_l_up_transport_parameters: Option<UPParameters>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBModifiedItemEUTRANIEExtensions>,
    }
    impl DRBModifiedItemEUTRAN {
        pub fn new(
            d_rb_id: DRBID,
            s1_dl_up_tnl_information: Option<UPTNLInformation>,
            p_dcp_sn_status_information: Option<PDCPSNStatusInformation>,
            u_l_up_transport_parameters: Option<UPParameters>,
            i_e_extensions: Option<DRBModifiedItemEUTRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                s1_dl_up_tnl_information,
                p_dcp_sn_status_information,
                u_l_up_transport_parameters,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBModifiedItemNGRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBModifiedItemNGRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBModifiedItemNGRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBModifiedItemNGRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBModifiedItemNGRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBModifiedItemNGRANIEExtensions(
        pub SequenceOf<AnonymousDRBModifiedItemNGRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-Modified-Item-NG-RAN")]
    #[non_exhaustive]
    pub struct DRBModifiedItemNGRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "uL-UP-Transport-Parameters")]
        pub u_l_up_transport_parameters: Option<UPParameters>,
        #[rasn(identifier = "pDCP-SN-Status-Information")]
        pub p_dcp_sn_status_information: Option<PDCPSNStatusInformation>,
        #[rasn(identifier = "flow-Setup-List")]
        pub flow_setup_list: Option<QoSFlowList>,
        #[rasn(identifier = "flow-Failed-List")]
        pub flow_failed_list: Option<QoSFlowFailedList>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBModifiedItemNGRANIEExtensions>,
    }
    impl DRBModifiedItemNGRAN {
        pub fn new(
            d_rb_id: DRBID,
            u_l_up_transport_parameters: Option<UPParameters>,
            p_dcp_sn_status_information: Option<PDCPSNStatusInformation>,
            flow_setup_list: Option<QoSFlowList>,
            flow_failed_list: Option<QoSFlowFailedList>,
            i_e_extensions: Option<DRBModifiedItemNGRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                u_l_up_transport_parameters,
                p_dcp_sn_status_information,
                flow_setup_list,
                flow_failed_list,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "DRB-Modified-List-EUTRAN")]
    pub struct DRBModifiedListEUTRAN(pub SequenceOf<DRBModifiedItemEUTRAN>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "DRB-Modified-List-NG-RAN")]
    pub struct DRBModifiedListNGRAN(pub SequenceOf<DRBModifiedItemNGRAN>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum DRBRemovedItemDRBReleasedInSession {
        #[rasn(identifier = "released-in-session")]
        released_in_session = 0,
        #[rasn(identifier = "not-released-in-session")]
        not_released_in_session = 1,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBRemovedItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBRemovedItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBRemovedItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBRemovedItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBRemovedItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBRemovedItemIEExtensions(pub SequenceOf<AnonymousDRBRemovedItemIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-Removed-Item")]
    #[non_exhaustive]
    pub struct DRBRemovedItem {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "dRB-Released-In-Session")]
        pub d_rb_released_in_session: Option<DRBRemovedItemDRBReleasedInSession>,
        #[rasn(size("5"), identifier = "dRB-Accumulated-Session-Time")]
        pub d_rb_accumulated_session_time: Option<OctetString>,
        #[rasn(size("1..=64"), identifier = "qoS-Flow-Removed-List")]
        pub qo_s_flow_removed_list: Option<SequenceOf<QoSFlowRemovedItem>>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBRemovedItemIEExtensions>,
    }
    impl DRBRemovedItem {
        pub fn new(
            d_rb_id: DRBID,
            d_rb_released_in_session: Option<DRBRemovedItemDRBReleasedInSession>,
            d_rb_accumulated_session_time: Option<OctetString>,
            qo_s_flow_removed_list: Option<SequenceOf<QoSFlowRemovedItem>>,
            i_e_extensions: Option<DRBRemovedItemIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                d_rb_released_in_session,
                d_rb_accumulated_session_time,
                qo_s_flow_removed_list,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBRequiredToModifyItemEUTRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBRequiredToModifyItemEUTRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBRequiredToModifyItemEUTRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBRequiredToModifyItemEUTRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBRequiredToModifyItemEUTRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBRequiredToModifyItemEUTRANIEExtensions(
        pub SequenceOf<AnonymousDRBRequiredToModifyItemEUTRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-Required-To-Modify-Item-EUTRAN")]
    #[non_exhaustive]
    pub struct DRBRequiredToModifyItemEUTRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "s1-DL-UP-TNL-Information")]
        pub s1_dl_up_tnl_information: Option<UPTNLInformation>,
        #[rasn(identifier = "gNB-CU-UP-CellGroupRelatedConfiguration")]
        pub g_nb_cu_up_cell_group_related_configuration:
            Option<GNBCUUPCellGroupRelatedConfiguration>,
        pub cause: Option<Cause>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBRequiredToModifyItemEUTRANIEExtensions>,
    }
    impl DRBRequiredToModifyItemEUTRAN {
        pub fn new(
            d_rb_id: DRBID,
            s1_dl_up_tnl_information: Option<UPTNLInformation>,
            g_nb_cu_up_cell_group_related_configuration: Option<
                GNBCUUPCellGroupRelatedConfiguration,
            >,
            cause: Option<Cause>,
            i_e_extensions: Option<DRBRequiredToModifyItemEUTRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                s1_dl_up_tnl_information,
                g_nb_cu_up_cell_group_related_configuration,
                cause,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBRequiredToModifyItemNGRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBRequiredToModifyItemNGRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBRequiredToModifyItemNGRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBRequiredToModifyItemNGRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBRequiredToModifyItemNGRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBRequiredToModifyItemNGRANIEExtensions(
        pub SequenceOf<AnonymousDRBRequiredToModifyItemNGRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-Required-To-Modify-Item-NG-RAN")]
    #[non_exhaustive]
    pub struct DRBRequiredToModifyItemNGRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "gNB-CU-UP-CellGroupRelatedConfiguration")]
        pub g_nb_cu_up_cell_group_related_configuration:
            Option<GNBCUUPCellGroupRelatedConfiguration>,
        #[rasn(identifier = "flow-To-Remove")]
        pub flow_to_remove: Option<QoSFlowList>,
        pub cause: Option<Cause>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBRequiredToModifyItemNGRANIEExtensions>,
    }
    impl DRBRequiredToModifyItemNGRAN {
        pub fn new(
            d_rb_id: DRBID,
            g_nb_cu_up_cell_group_related_configuration: Option<
                GNBCUUPCellGroupRelatedConfiguration,
            >,
            flow_to_remove: Option<QoSFlowList>,
            cause: Option<Cause>,
            i_e_extensions: Option<DRBRequiredToModifyItemNGRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                g_nb_cu_up_cell_group_related_configuration,
                flow_to_remove,
                cause,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=32"),
        identifier = "DRB-Required-To-Modify-List-EUTRAN"
    )]
    pub struct DRBRequiredToModifyListEUTRAN(pub SequenceOf<DRBRequiredToModifyItemEUTRAN>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=32"),
        identifier = "DRB-Required-To-Modify-List-NG-RAN"
    )]
    pub struct DRBRequiredToModifyListNGRAN(pub SequenceOf<DRBRequiredToModifyItemNGRAN>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBRequiredToRemoveItemEUTRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBRequiredToRemoveItemEUTRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBRequiredToRemoveItemEUTRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBRequiredToRemoveItemEUTRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBRequiredToRemoveItemEUTRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBRequiredToRemoveItemEUTRANIEExtensions(
        pub SequenceOf<AnonymousDRBRequiredToRemoveItemEUTRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-Required-To-Remove-Item-EUTRAN")]
    #[non_exhaustive]
    pub struct DRBRequiredToRemoveItemEUTRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        pub cause: Cause,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBRequiredToRemoveItemEUTRANIEExtensions>,
    }
    impl DRBRequiredToRemoveItemEUTRAN {
        pub fn new(
            d_rb_id: DRBID,
            cause: Cause,
            i_e_extensions: Option<DRBRequiredToRemoveItemEUTRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                cause,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBRequiredToRemoveItemNGRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBRequiredToRemoveItemNGRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBRequiredToRemoveItemNGRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBRequiredToRemoveItemNGRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBRequiredToRemoveItemNGRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBRequiredToRemoveItemNGRANIEExtensions(
        pub SequenceOf<AnonymousDRBRequiredToRemoveItemNGRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-Required-To-Remove-Item-NG-RAN")]
    #[non_exhaustive]
    pub struct DRBRequiredToRemoveItemNGRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        pub cause: Cause,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBRequiredToRemoveItemNGRANIEExtensions>,
    }
    impl DRBRequiredToRemoveItemNGRAN {
        pub fn new(
            d_rb_id: DRBID,
            cause: Cause,
            i_e_extensions: Option<DRBRequiredToRemoveItemNGRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                cause,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=32"),
        identifier = "DRB-Required-To-Remove-List-EUTRAN"
    )]
    pub struct DRBRequiredToRemoveListEUTRAN(pub SequenceOf<DRBRequiredToRemoveItemEUTRAN>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=32"),
        identifier = "DRB-Required-To-Remove-List-NG-RAN"
    )]
    pub struct DRBRequiredToRemoveListNGRAN(pub SequenceOf<DRBRequiredToRemoveItemNGRAN>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum DRBSetupItemEUTRANS1DLUPUnchanged {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBSetupItemEUTRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBSetupItemEUTRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBSetupItemEUTRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBSetupItemEUTRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBSetupItemEUTRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBSetupItemEUTRANIEExtensions(
        pub SequenceOf<AnonymousDRBSetupItemEUTRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-Setup-Item-EUTRAN")]
    #[non_exhaustive]
    pub struct DRBSetupItemEUTRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "s1-DL-UP-TNL-Information")]
        pub s1_dl_up_tnl_information: UPTNLInformation,
        #[rasn(identifier = "data-Forwarding-Information-Response")]
        pub data_forwarding_information_response: Option<DataForwardingInformation>,
        #[rasn(identifier = "uL-UP-Transport-Parameters")]
        pub u_l_up_transport_parameters: UPParameters,
        #[rasn(identifier = "s1-DL-UP-Unchanged")]
        pub s1_dl_up_unchanged: Option<DRBSetupItemEUTRANS1DLUPUnchanged>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBSetupItemEUTRANIEExtensions>,
    }
    impl DRBSetupItemEUTRAN {
        pub fn new(
            d_rb_id: DRBID,
            s1_dl_up_tnl_information: UPTNLInformation,
            data_forwarding_information_response: Option<DataForwardingInformation>,
            u_l_up_transport_parameters: UPParameters,
            s1_dl_up_unchanged: Option<DRBSetupItemEUTRANS1DLUPUnchanged>,
            i_e_extensions: Option<DRBSetupItemEUTRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                s1_dl_up_tnl_information,
                data_forwarding_information_response,
                u_l_up_transport_parameters,
                s1_dl_up_unchanged,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBSetupItemNGRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBSetupItemNGRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBSetupItemNGRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBSetupItemNGRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBSetupItemNGRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBSetupItemNGRANIEExtensions(
        pub SequenceOf<AnonymousDRBSetupItemNGRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-Setup-Item-NG-RAN")]
    #[non_exhaustive]
    pub struct DRBSetupItemNGRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "dRB-data-Forwarding-Information-Response")]
        pub d_rb_data_forwarding_information_response: Option<DataForwardingInformation>,
        #[rasn(identifier = "uL-UP-Transport-Parameters")]
        pub u_l_up_transport_parameters: UPParameters,
        #[rasn(identifier = "flow-Setup-List")]
        pub flow_setup_list: QoSFlowList,
        #[rasn(identifier = "flow-Failed-List")]
        pub flow_failed_list: Option<QoSFlowFailedList>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBSetupItemNGRANIEExtensions>,
    }
    impl DRBSetupItemNGRAN {
        pub fn new(
            d_rb_id: DRBID,
            d_rb_data_forwarding_information_response: Option<DataForwardingInformation>,
            u_l_up_transport_parameters: UPParameters,
            flow_setup_list: QoSFlowList,
            flow_failed_list: Option<QoSFlowFailedList>,
            i_e_extensions: Option<DRBSetupItemNGRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                d_rb_data_forwarding_information_response,
                u_l_up_transport_parameters,
                flow_setup_list,
                flow_failed_list,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "DRB-Setup-List-EUTRAN")]
    pub struct DRBSetupListEUTRAN(pub SequenceOf<DRBSetupItemEUTRAN>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "DRB-Setup-List-NG-RAN")]
    pub struct DRBSetupListNGRAN(pub SequenceOf<DRBSetupItemNGRAN>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBSetupModItemEUTRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBSetupModItemEUTRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBSetupModItemEUTRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBSetupModItemEUTRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBSetupModItemEUTRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBSetupModItemEUTRANIEExtensions(
        pub SequenceOf<AnonymousDRBSetupModItemEUTRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-Setup-Mod-Item-EUTRAN")]
    #[non_exhaustive]
    pub struct DRBSetupModItemEUTRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "s1-DL-UP-TNL-Information")]
        pub s1_dl_up_tnl_information: UPTNLInformation,
        #[rasn(identifier = "data-Forwarding-Information-Response")]
        pub data_forwarding_information_response: Option<DataForwardingInformation>,
        #[rasn(identifier = "uL-UP-Transport-Parameters")]
        pub u_l_up_transport_parameters: UPParameters,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBSetupModItemEUTRANIEExtensions>,
    }
    impl DRBSetupModItemEUTRAN {
        pub fn new(
            d_rb_id: DRBID,
            s1_dl_up_tnl_information: UPTNLInformation,
            data_forwarding_information_response: Option<DataForwardingInformation>,
            u_l_up_transport_parameters: UPParameters,
            i_e_extensions: Option<DRBSetupModItemEUTRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                s1_dl_up_tnl_information,
                data_forwarding_information_response,
                u_l_up_transport_parameters,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBSetupModItemNGRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBSetupModItemNGRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBSetupModItemNGRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBSetupModItemNGRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBSetupModItemNGRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBSetupModItemNGRANIEExtensions(
        pub SequenceOf<AnonymousDRBSetupModItemNGRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-Setup-Mod-Item-NG-RAN")]
    #[non_exhaustive]
    pub struct DRBSetupModItemNGRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "dRB-data-Forwarding-Information-Response")]
        pub d_rb_data_forwarding_information_response: Option<DataForwardingInformation>,
        #[rasn(identifier = "uL-UP-Transport-Parameters")]
        pub u_l_up_transport_parameters: UPParameters,
        #[rasn(identifier = "flow-Setup-List")]
        pub flow_setup_list: QoSFlowList,
        #[rasn(identifier = "flow-Failed-List")]
        pub flow_failed_list: Option<QoSFlowFailedList>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBSetupModItemNGRANIEExtensions>,
    }
    impl DRBSetupModItemNGRAN {
        pub fn new(
            d_rb_id: DRBID,
            d_rb_data_forwarding_information_response: Option<DataForwardingInformation>,
            u_l_up_transport_parameters: UPParameters,
            flow_setup_list: QoSFlowList,
            flow_failed_list: Option<QoSFlowFailedList>,
            i_e_extensions: Option<DRBSetupModItemNGRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                d_rb_data_forwarding_information_response,
                u_l_up_transport_parameters,
                flow_setup_list,
                flow_failed_list,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "DRB-Setup-Mod-List-EUTRAN")]
    pub struct DRBSetupModListEUTRAN(pub SequenceOf<DRBSetupModItemEUTRAN>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "DRB-Setup-Mod-List-NG-RAN")]
    pub struct DRBSetupModListNGRAN(pub SequenceOf<DRBSetupModItemNGRAN>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBStatusItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBStatusItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBStatusItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBStatusItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBStatusItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBStatusItemIEExtensions(pub SequenceOf<AnonymousDRBStatusItemIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-Status-Item")]
    #[non_exhaustive]
    pub struct DRBStatusItem {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "pDCP-DL-Count")]
        pub p_dcp_dl_count: Option<PDCPCount>,
        #[rasn(identifier = "pDCP-UL-Count")]
        pub p_dcp_ul_count: Option<PDCPCount>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBStatusItemIEExtensions>,
    }
    impl DRBStatusItem {
        pub fn new(
            d_rb_id: DRBID,
            p_dcp_dl_count: Option<PDCPCount>,
            p_dcp_ul_count: Option<PDCPCount>,
            i_e_extensions: Option<DRBStatusItemIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                p_dcp_dl_count,
                p_dcp_ul_count,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBToModifyItemEUTRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBToModifyItemEUTRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBToModifyItemEUTRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBToModifyItemEUTRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBToModifyItemEUTRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBToModifyItemEUTRANIEExtensions(
        pub SequenceOf<AnonymousDRBToModifyItemEUTRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-To-Modify-Item-EUTRAN")]
    #[non_exhaustive]
    pub struct DRBToModifyItemEUTRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "pDCP-Configuration")]
        pub p_dcp_configuration: Option<PDCPConfiguration>,
        #[rasn(identifier = "eUTRAN-QoS")]
        pub e_utran_qo_s: Option<EUTRANQoS>,
        #[rasn(identifier = "s1-UL-UP-TNL-Information")]
        pub s1_ul_up_tnl_information: Option<UPTNLInformation>,
        #[rasn(identifier = "data-Forwarding-Information")]
        pub data_forwarding_information: Option<DataForwardingInformation>,
        #[rasn(identifier = "pDCP-SN-Status-Request")]
        pub p_dcp_sn_status_request: Option<PDCPSNStatusRequest>,
        #[rasn(identifier = "pDCP-SN-Status-Information")]
        pub p_dcp_sn_status_information: Option<PDCPSNStatusInformation>,
        #[rasn(identifier = "dL-UP-Parameters")]
        pub d_l_up_parameters: Option<UPParameters>,
        #[rasn(identifier = "cell-Group-To-Add")]
        pub cell_group_to_add: Option<CellGroupInformation>,
        #[rasn(identifier = "cell-Group-To-Modify")]
        pub cell_group_to_modify: Option<CellGroupInformation>,
        #[rasn(identifier = "cell-Group-To-Remove")]
        pub cell_group_to_remove: Option<CellGroupInformation>,
        #[rasn(identifier = "dRB-Inactivity-Timer")]
        pub d_rb_inactivity_timer: Option<InactivityTimer>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBToModifyItemEUTRANIEExtensions>,
    }
    impl DRBToModifyItemEUTRAN {
        pub fn new(
            d_rb_id: DRBID,
            p_dcp_configuration: Option<PDCPConfiguration>,
            e_utran_qo_s: Option<EUTRANQoS>,
            s1_ul_up_tnl_information: Option<UPTNLInformation>,
            data_forwarding_information: Option<DataForwardingInformation>,
            p_dcp_sn_status_request: Option<PDCPSNStatusRequest>,
            p_dcp_sn_status_information: Option<PDCPSNStatusInformation>,
            d_l_up_parameters: Option<UPParameters>,
            cell_group_to_add: Option<CellGroupInformation>,
            cell_group_to_modify: Option<CellGroupInformation>,
            cell_group_to_remove: Option<CellGroupInformation>,
            d_rb_inactivity_timer: Option<InactivityTimer>,
            i_e_extensions: Option<DRBToModifyItemEUTRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                p_dcp_configuration,
                e_utran_qo_s,
                s1_ul_up_tnl_information,
                data_forwarding_information,
                p_dcp_sn_status_request,
                p_dcp_sn_status_information,
                d_l_up_parameters,
                cell_group_to_add,
                cell_group_to_modify,
                cell_group_to_remove,
                d_rb_inactivity_timer,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBToModifyItemNGRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBToModifyItemNGRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBToModifyItemNGRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBToModifyItemNGRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBToModifyItemNGRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBToModifyItemNGRANIEExtensions(
        pub SequenceOf<AnonymousDRBToModifyItemNGRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-To-Modify-Item-NG-RAN")]
    #[non_exhaustive]
    pub struct DRBToModifyItemNGRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "sDAP-Configuration")]
        pub s_dap_configuration: Option<SDAPConfiguration>,
        #[rasn(identifier = "pDCP-Configuration")]
        pub p_dcp_configuration: Option<PDCPConfiguration>,
        #[rasn(identifier = "dRB-Data-Forwarding-Information")]
        pub d_rb_data_forwarding_information: Option<DataForwardingInformation>,
        #[rasn(identifier = "pDCP-SN-Status-Request")]
        pub p_dcp_sn_status_request: Option<PDCPSNStatusRequest>,
        #[rasn(identifier = "pdcp-SN-Status-Information")]
        pub pdcp_sn_status_information: Option<PDCPSNStatusInformation>,
        #[rasn(identifier = "dL-UP-Parameters")]
        pub d_l_up_parameters: Option<UPParameters>,
        #[rasn(identifier = "cell-Group-To-Add")]
        pub cell_group_to_add: Option<CellGroupInformation>,
        #[rasn(identifier = "cell-Group-To-Modify")]
        pub cell_group_to_modify: Option<CellGroupInformation>,
        #[rasn(identifier = "cell-Group-To-Remove")]
        pub cell_group_to_remove: Option<CellGroupInformation>,
        #[rasn(identifier = "flow-Mapping-Information")]
        pub flow_mapping_information: Option<QoSFlowQoSParameterList>,
        #[rasn(identifier = "dRB-Inactivity-Timer")]
        pub d_rb_inactivity_timer: Option<InactivityTimer>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBToModifyItemNGRANIEExtensions>,
    }
    impl DRBToModifyItemNGRAN {
        pub fn new(
            d_rb_id: DRBID,
            s_dap_configuration: Option<SDAPConfiguration>,
            p_dcp_configuration: Option<PDCPConfiguration>,
            d_rb_data_forwarding_information: Option<DataForwardingInformation>,
            p_dcp_sn_status_request: Option<PDCPSNStatusRequest>,
            pdcp_sn_status_information: Option<PDCPSNStatusInformation>,
            d_l_up_parameters: Option<UPParameters>,
            cell_group_to_add: Option<CellGroupInformation>,
            cell_group_to_modify: Option<CellGroupInformation>,
            cell_group_to_remove: Option<CellGroupInformation>,
            flow_mapping_information: Option<QoSFlowQoSParameterList>,
            d_rb_inactivity_timer: Option<InactivityTimer>,
            i_e_extensions: Option<DRBToModifyItemNGRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                s_dap_configuration,
                p_dcp_configuration,
                d_rb_data_forwarding_information,
                p_dcp_sn_status_request,
                pdcp_sn_status_information,
                d_l_up_parameters,
                cell_group_to_add,
                cell_group_to_modify,
                cell_group_to_remove,
                flow_mapping_information,
                d_rb_inactivity_timer,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "DRB-To-Modify-List-EUTRAN")]
    pub struct DRBToModifyListEUTRAN(pub SequenceOf<DRBToModifyItemEUTRAN>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "DRB-To-Modify-List-NG-RAN")]
    pub struct DRBToModifyListNGRAN(pub SequenceOf<DRBToModifyItemNGRAN>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBToRemoveItemEUTRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBToRemoveItemEUTRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBToRemoveItemEUTRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBToRemoveItemEUTRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBToRemoveItemEUTRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBToRemoveItemEUTRANIEExtensions(
        pub SequenceOf<AnonymousDRBToRemoveItemEUTRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-To-Remove-Item-EUTRAN")]
    #[non_exhaustive]
    pub struct DRBToRemoveItemEUTRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBToRemoveItemEUTRANIEExtensions>,
    }
    impl DRBToRemoveItemEUTRAN {
        pub fn new(
            d_rb_id: DRBID,
            i_e_extensions: Option<DRBToRemoveItemEUTRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBToRemoveItemNGRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBToRemoveItemNGRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBToRemoveItemNGRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBToRemoveItemNGRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBToRemoveItemNGRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBToRemoveItemNGRANIEExtensions(
        pub SequenceOf<AnonymousDRBToRemoveItemNGRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-To-Remove-Item-NG-RAN")]
    #[non_exhaustive]
    pub struct DRBToRemoveItemNGRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBToRemoveItemNGRANIEExtensions>,
    }
    impl DRBToRemoveItemNGRAN {
        pub fn new(
            d_rb_id: DRBID,
            i_e_extensions: Option<DRBToRemoveItemNGRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "DRB-To-Remove-List-EUTRAN")]
    pub struct DRBToRemoveListEUTRAN(pub SequenceOf<DRBToRemoveItemEUTRAN>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "DRB-To-Remove-List-NG-RAN")]
    pub struct DRBToRemoveListNGRAN(pub SequenceOf<DRBToRemoveItemNGRAN>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBToSetupItemEUTRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBToSetupItemEUTRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBToSetupItemEUTRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBToSetupItemEUTRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBToSetupItemEUTRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBToSetupItemEUTRANIEExtensions(
        pub SequenceOf<AnonymousDRBToSetupItemEUTRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-To-Setup-Item-EUTRAN")]
    #[non_exhaustive]
    pub struct DRBToSetupItemEUTRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "pDCP-Configuration")]
        pub p_dcp_configuration: PDCPConfiguration,
        #[rasn(identifier = "eUTRAN-QoS")]
        pub e_utran_qo_s: EUTRANQoS,
        #[rasn(identifier = "s1-UL-UP-TNL-Information")]
        pub s1_ul_up_tnl_information: UPTNLInformation,
        #[rasn(identifier = "data-Forwarding-Information-Request")]
        pub data_forwarding_information_request: Option<DataForwardingInformationRequest>,
        #[rasn(identifier = "cell-Group-Information")]
        pub cell_group_information: CellGroupInformation,
        #[rasn(identifier = "dL-UP-Parameters")]
        pub d_l_up_parameters: Option<UPParameters>,
        #[rasn(identifier = "dRB-Inactivity-Timer")]
        pub d_rb_inactivity_timer: Option<InactivityTimer>,
        #[rasn(identifier = "existing-Allocated-S1-DL-UP-TNL-Info")]
        pub existing_allocated_s1_dl_up_tnl_info: Option<UPTNLInformation>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBToSetupItemEUTRANIEExtensions>,
    }
    impl DRBToSetupItemEUTRAN {
        pub fn new(
            d_rb_id: DRBID,
            p_dcp_configuration: PDCPConfiguration,
            e_utran_qo_s: EUTRANQoS,
            s1_ul_up_tnl_information: UPTNLInformation,
            data_forwarding_information_request: Option<DataForwardingInformationRequest>,
            cell_group_information: CellGroupInformation,
            d_l_up_parameters: Option<UPParameters>,
            d_rb_inactivity_timer: Option<InactivityTimer>,
            existing_allocated_s1_dl_up_tnl_info: Option<UPTNLInformation>,
            i_e_extensions: Option<DRBToSetupItemEUTRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                p_dcp_configuration,
                e_utran_qo_s,
                s1_ul_up_tnl_information,
                data_forwarding_information_request,
                cell_group_information,
                d_l_up_parameters,
                d_rb_inactivity_timer,
                existing_allocated_s1_dl_up_tnl_info,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBToSetupItemNGRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBToSetupItemNGRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBToSetupItemNGRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBToSetupItemNGRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBToSetupItemNGRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBToSetupItemNGRANIEExtensions(
        pub SequenceOf<AnonymousDRBToSetupItemNGRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-To-Setup-Item-NG-RAN")]
    #[non_exhaustive]
    pub struct DRBToSetupItemNGRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "sDAP-Configuration")]
        pub s_dap_configuration: SDAPConfiguration,
        #[rasn(identifier = "pDCP-Configuration")]
        pub p_dcp_configuration: PDCPConfiguration,
        #[rasn(identifier = "cell-Group-Information")]
        pub cell_group_information: CellGroupInformation,
        #[rasn(identifier = "qos-flow-Information-To-Be-Setup")]
        pub qos_flow_information_to_be_setup: QoSFlowQoSParameterList,
        #[rasn(identifier = "dRB-Data-Forwarding-Information-Request")]
        pub d_rb_data_forwarding_information_request: Option<DataForwardingInformationRequest>,
        #[rasn(identifier = "dRB-Inactivity-Timer")]
        pub d_rb_inactivity_timer: Option<InactivityTimer>,
        #[rasn(identifier = "pDCP-SN-Status-Information")]
        pub p_dcp_sn_status_information: Option<PDCPSNStatusInformation>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBToSetupItemNGRANIEExtensions>,
    }
    impl DRBToSetupItemNGRAN {
        pub fn new(
            d_rb_id: DRBID,
            s_dap_configuration: SDAPConfiguration,
            p_dcp_configuration: PDCPConfiguration,
            cell_group_information: CellGroupInformation,
            qos_flow_information_to_be_setup: QoSFlowQoSParameterList,
            d_rb_data_forwarding_information_request: Option<DataForwardingInformationRequest>,
            d_rb_inactivity_timer: Option<InactivityTimer>,
            p_dcp_sn_status_information: Option<PDCPSNStatusInformation>,
            i_e_extensions: Option<DRBToSetupItemNGRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                s_dap_configuration,
                p_dcp_configuration,
                cell_group_information,
                qos_flow_information_to_be_setup,
                d_rb_data_forwarding_information_request,
                d_rb_inactivity_timer,
                p_dcp_sn_status_information,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "DRB-To-Setup-List-EUTRAN")]
    pub struct DRBToSetupListEUTRAN(pub SequenceOf<DRBToSetupItemEUTRAN>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "DRB-To-Setup-List-NG-RAN")]
    pub struct DRBToSetupListNGRAN(pub SequenceOf<DRBToSetupItemNGRAN>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBToSetupModItemEUTRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBToSetupModItemEUTRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBToSetupModItemEUTRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBToSetupModItemEUTRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBToSetupModItemEUTRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBToSetupModItemEUTRANIEExtensions(
        pub SequenceOf<AnonymousDRBToSetupModItemEUTRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-To-Setup-Mod-Item-EUTRAN")]
    #[non_exhaustive]
    pub struct DRBToSetupModItemEUTRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "pDCP-Configuration")]
        pub p_dcp_configuration: PDCPConfiguration,
        #[rasn(identifier = "eUTRAN-QoS")]
        pub e_utran_qo_s: EUTRANQoS,
        #[rasn(identifier = "s1-UL-UP-TNL-Information")]
        pub s1_ul_up_tnl_information: UPTNLInformation,
        #[rasn(identifier = "data-Forwarding-Information-Request")]
        pub data_forwarding_information_request: Option<DataForwardingInformationRequest>,
        #[rasn(identifier = "cell-Group-Information")]
        pub cell_group_information: CellGroupInformation,
        #[rasn(identifier = "dL-UP-Parameters")]
        pub d_l_up_parameters: Option<UPParameters>,
        #[rasn(identifier = "dRB-Inactivity-Timer")]
        pub d_rb_inactivity_timer: Option<InactivityTimer>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBToSetupModItemEUTRANIEExtensions>,
    }
    impl DRBToSetupModItemEUTRAN {
        pub fn new(
            d_rb_id: DRBID,
            p_dcp_configuration: PDCPConfiguration,
            e_utran_qo_s: EUTRANQoS,
            s1_ul_up_tnl_information: UPTNLInformation,
            data_forwarding_information_request: Option<DataForwardingInformationRequest>,
            cell_group_information: CellGroupInformation,
            d_l_up_parameters: Option<UPParameters>,
            d_rb_inactivity_timer: Option<InactivityTimer>,
            i_e_extensions: Option<DRBToSetupModItemEUTRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                p_dcp_configuration,
                e_utran_qo_s,
                s1_ul_up_tnl_information,
                data_forwarding_information_request,
                cell_group_information,
                d_l_up_parameters,
                d_rb_inactivity_timer,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBToSetupModItemNGRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBToSetupModItemNGRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBToSetupModItemNGRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBToSetupModItemNGRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBToSetupModItemNGRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBToSetupModItemNGRANIEExtensions(
        pub SequenceOf<AnonymousDRBToSetupModItemNGRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-To-Setup-Mod-Item-NG-RAN")]
    #[non_exhaustive]
    pub struct DRBToSetupModItemNGRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "sDAP-Configuration")]
        pub s_dap_configuration: SDAPConfiguration,
        #[rasn(identifier = "pDCP-Configuration")]
        pub p_dcp_configuration: PDCPConfiguration,
        #[rasn(identifier = "cell-Group-Information")]
        pub cell_group_information: CellGroupInformation,
        #[rasn(identifier = "flow-Mapping-Information")]
        pub flow_mapping_information: QoSFlowQoSParameterList,
        #[rasn(identifier = "dRB-Data-Forwarding-Information-Request")]
        pub d_rb_data_forwarding_information_request: Option<DataForwardingInformationRequest>,
        #[rasn(identifier = "dRB-Inactivity-Timer")]
        pub d_rb_inactivity_timer: Option<InactivityTimer>,
        #[rasn(identifier = "pDCP-SN-Status-Information")]
        pub p_dcp_sn_status_information: Option<PDCPSNStatusInformation>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBToSetupModItemNGRANIEExtensions>,
    }
    impl DRBToSetupModItemNGRAN {
        pub fn new(
            d_rb_id: DRBID,
            s_dap_configuration: SDAPConfiguration,
            p_dcp_configuration: PDCPConfiguration,
            cell_group_information: CellGroupInformation,
            flow_mapping_information: QoSFlowQoSParameterList,
            d_rb_data_forwarding_information_request: Option<DataForwardingInformationRequest>,
            d_rb_inactivity_timer: Option<InactivityTimer>,
            p_dcp_sn_status_information: Option<PDCPSNStatusInformation>,
            i_e_extensions: Option<DRBToSetupModItemNGRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                s_dap_configuration,
                p_dcp_configuration,
                cell_group_information,
                flow_mapping_information,
                d_rb_data_forwarding_information_request,
                d_rb_inactivity_timer,
                p_dcp_sn_status_information,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "DRB-To-Setup-Mod-List-EUTRAN")]
    pub struct DRBToSetupModListEUTRAN(pub SequenceOf<DRBToSetupModItemEUTRAN>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "DRB-To-Setup-Mod-List-NG-RAN")]
    pub struct DRBToSetupModListNGRAN(pub SequenceOf<DRBToSetupModItemNGRAN>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBUsageReportItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBUsageReportItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBUsageReportItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBUsageReportItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBUsageReportItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBUsageReportItemIEExtensions(
        pub SequenceOf<AnonymousDRBUsageReportItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRB-Usage-Report-Item")]
    #[non_exhaustive]
    pub struct DRBUsageReportItem {
        #[rasn(size("4"), identifier = "startTimeStamp")]
        pub start_time_stamp: OctetString,
        #[rasn(size("4"), identifier = "endTimeStamp")]
        pub end_time_stamp: OctetString,
        #[rasn(value("0..=18446744073709551615"), identifier = "usageCountUL")]
        pub usage_count_ul: u64,
        #[rasn(value("0..=18446744073709551615"), identifier = "usageCountDL")]
        pub usage_count_dl: u64,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBUsageReportItemIEExtensions>,
    }
    impl DRBUsageReportItem {
        pub fn new(
            start_time_stamp: OctetString,
            end_time_stamp: OctetString,
            usage_count_ul: u64,
            usage_count_dl: u64,
            i_e_extensions: Option<DRBUsageReportItemIEExtensions>,
        ) -> Self {
            Self {
                start_time_stamp,
                end_time_stamp,
                usage_count_ul,
                usage_count_dl,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=2"), identifier = "DRB-Usage-Report-List")]
    pub struct DRBUsageReportList(pub SequenceOf<DRBUsageReportItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBBStatusTransferIEExtensionCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBBStatusTransferIEExtension {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBBStatusTransferIEExtensionCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBBStatusTransferIEExtension {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBBStatusTransferIEExtensionCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBBStatusTransferIEExtension(
        pub SequenceOf<AnonymousDRBBStatusTransferIEExtension>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct DRBBStatusTransfer {
        #[rasn(size("1..=131072"), identifier = "receiveStatusofPDCPSDU")]
        pub receive_statusof_pdcpsdu: Option<BitString>,
        #[rasn(identifier = "countValue")]
        pub count_value: PDCPCount,
        #[rasn(identifier = "iE-Extension")]
        pub i_e_extension: Option<DRBBStatusTransferIEExtension>,
    }
    impl DRBBStatusTransfer {
        pub fn new(
            receive_statusof_pdcpsdu: Option<BitString>,
            count_value: PDCPCount,
            i_e_extension: Option<DRBBStatusTransferIEExtension>,
        ) -> Self {
            Self {
                receive_statusof_pdcpsdu,
                count_value,
                i_e_extension,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBsSubjectToCounterCheckItemEUTRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBsSubjectToCounterCheckItemEUTRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBsSubjectToCounterCheckItemEUTRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBsSubjectToCounterCheckItemEUTRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBsSubjectToCounterCheckItemEUTRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBsSubjectToCounterCheckItemEUTRANIEExtensions(
        pub SequenceOf<AnonymousDRBsSubjectToCounterCheckItemEUTRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "DRBs-Subject-To-Counter-Check-Item-EUTRAN"
    )]
    #[non_exhaustive]
    pub struct DRBsSubjectToCounterCheckItemEUTRAN {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "pDCP-UL-Count")]
        pub p_dcp_ul_count: PDCPCount,
        #[rasn(identifier = "pDCP-DL-Count")]
        pub p_dcp_dl_count: PDCPCount,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBsSubjectToCounterCheckItemEUTRANIEExtensions>,
    }
    impl DRBsSubjectToCounterCheckItemEUTRAN {
        pub fn new(
            d_rb_id: DRBID,
            p_dcp_ul_count: PDCPCount,
            p_dcp_dl_count: PDCPCount,
            i_e_extensions: Option<DRBsSubjectToCounterCheckItemEUTRANIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                p_dcp_ul_count,
                p_dcp_dl_count,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBsSubjectToCounterCheckItemNGRANIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBsSubjectToCounterCheckItemNGRANIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBsSubjectToCounterCheckItemNGRANIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBsSubjectToCounterCheckItemNGRANIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBsSubjectToCounterCheckItemNGRANIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBsSubjectToCounterCheckItemNGRANIEExtensions(
        pub SequenceOf<AnonymousDRBsSubjectToCounterCheckItemNGRANIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "DRBs-Subject-To-Counter-Check-Item-NG-RAN"
    )]
    #[non_exhaustive]
    pub struct DRBsSubjectToCounterCheckItemNGRAN {
        #[rasn(identifier = "pDU-Session-ID")]
        pub p_du_session_id: PDUSessionID,
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "pDCP-UL-Count")]
        pub p_dcp_ul_count: PDCPCount,
        #[rasn(identifier = "pDCP-DL-Count")]
        pub p_dcp_dl_count: PDCPCount,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBsSubjectToCounterCheckItemNGRANIEExtensions>,
    }
    impl DRBsSubjectToCounterCheckItemNGRAN {
        pub fn new(
            p_du_session_id: PDUSessionID,
            d_rb_id: DRBID,
            p_dcp_ul_count: PDCPCount,
            p_dcp_dl_count: PDCPCount,
            i_e_extensions: Option<DRBsSubjectToCounterCheckItemNGRANIEExtensions>,
        ) -> Self {
            Self {
                p_du_session_id,
                d_rb_id,
                p_dcp_ul_count,
                p_dcp_dl_count,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=32"),
        identifier = "DRBs-Subject-To-Counter-Check-List-EUTRAN"
    )]
    pub struct DRBsSubjectToCounterCheckListEUTRAN(
        pub SequenceOf<DRBsSubjectToCounterCheckItemEUTRAN>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=32"),
        identifier = "DRBs-Subject-To-Counter-Check-List-NG-RAN"
    )]
    pub struct DRBsSubjectToCounterCheckListNGRAN(
        pub SequenceOf<DRBsSubjectToCounterCheckItemNGRAN>,
    );
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDRBsSubjectToEarlyForwardingItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDRBsSubjectToEarlyForwardingItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDRBsSubjectToEarlyForwardingItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDRBsSubjectToEarlyForwardingItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDRBsSubjectToEarlyForwardingItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DRBsSubjectToEarlyForwardingItemIEExtensions(
        pub SequenceOf<AnonymousDRBsSubjectToEarlyForwardingItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DRBs-Subject-To-Early-Forwarding-Item")]
    #[non_exhaustive]
    pub struct DRBsSubjectToEarlyForwardingItem {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "dLCountValue")]
        pub d_lcount_value: PDCPCount,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DRBsSubjectToEarlyForwardingItemIEExtensions>,
    }
    impl DRBsSubjectToEarlyForwardingItem {
        pub fn new(
            d_rb_id: DRBID,
            d_lcount_value: PDCPCount,
            i_e_extensions: Option<DRBsSubjectToEarlyForwardingItemIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                d_lcount_value,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=32"),
        identifier = "DRBs-Subject-To-Early-Forwarding-List"
    )]
    pub struct DRBsSubjectToEarlyForwardingList(pub SequenceOf<DRBsSubjectToEarlyForwardingItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDataForwardingInformationIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDataForwardingInformationIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDataForwardingInformationIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDataForwardingInformationIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDataForwardingInformationIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DataForwardingInformationIEExtensions(
        pub SequenceOf<AnonymousDataForwardingInformationIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Data-Forwarding-Information")]
    #[non_exhaustive]
    pub struct DataForwardingInformation {
        #[rasn(identifier = "uL-Data-Forwarding")]
        pub u_l_data_forwarding: Option<UPTNLInformation>,
        #[rasn(identifier = "dL-Data-Forwarding")]
        pub d_l_data_forwarding: Option<UPTNLInformation>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DataForwardingInformationIEExtensions>,
    }
    impl DataForwardingInformation {
        pub fn new(
            u_l_data_forwarding: Option<UPTNLInformation>,
            d_l_data_forwarding: Option<UPTNLInformation>,
            i_e_extensions: Option<DataForwardingInformationIEExtensions>,
        ) -> Self {
            Self {
                u_l_data_forwarding,
                d_l_data_forwarding,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDataForwardingInformationRequestIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDataForwardingInformationRequestIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDataForwardingInformationRequestIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDataForwardingInformationRequestIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDataForwardingInformationRequestIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DataForwardingInformationRequestIEExtensions(
        pub SequenceOf<AnonymousDataForwardingInformationRequestIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Data-Forwarding-Information-Request")]
    #[non_exhaustive]
    pub struct DataForwardingInformationRequest {
        #[rasn(identifier = "data-Forwarding-Request")]
        pub data_forwarding_request: DataForwardingRequest,
        #[rasn(identifier = "qoS-Flows-Forwarded-On-Fwd-Tunnels")]
        pub qo_s_flows_forwarded_on_fwd_tunnels: Option<QoSFlowMappingList>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DataForwardingInformationRequestIEExtensions>,
    }
    impl DataForwardingInformationRequest {
        pub fn new(
            data_forwarding_request: DataForwardingRequest,
            qo_s_flows_forwarded_on_fwd_tunnels: Option<QoSFlowMappingList>,
            i_e_extensions: Option<DataForwardingInformationRequestIEExtensions>,
        ) -> Self {
            Self {
                data_forwarding_request,
                qo_s_flows_forwarded_on_fwd_tunnels,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "Data-Forwarding-Request")]
    #[non_exhaustive]
    pub enum DataForwardingRequest {
        uL = 0,
        dL = 1,
        both = 2,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDataUsageReportItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDataUsageReportItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDataUsageReportItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDataUsageReportItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDataUsageReportItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DataUsageReportItemIEExtensions(
        pub SequenceOf<AnonymousDataUsageReportItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Data-Usage-Report-Item")]
    #[non_exhaustive]
    pub struct DataUsageReportItem {
        #[rasn(identifier = "dRB-ID")]
        pub d_rb_id: DRBID,
        #[rasn(identifier = "rAT-Type")]
        pub r_at_type: RATType,
        #[rasn(identifier = "dRB-Usage-Report-List")]
        pub d_rb_usage_report_list: DRBUsageReportList,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DataUsageReportItemIEExtensions>,
    }
    impl DataUsageReportItem {
        pub fn new(
            d_rb_id: DRBID,
            r_at_type: RATType,
            d_rb_usage_report_list: DRBUsageReportList,
            i_e_extensions: Option<DataUsageReportItemIEExtensions>,
        ) -> Self {
            Self {
                d_rb_id,
                r_at_type,
                d_rb_usage_report_list,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "Data-Usage-Report-List")]
    pub struct DataUsageReportList(pub SequenceOf<DataUsageReportItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum DataUsagePerPDUSessionReportSecondaryRATType {
        nR = 0,
        #[rasn(identifier = "e-UTRA")]
        e_UTRA = 1,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDataUsagePerPDUSessionReportIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDataUsagePerPDUSessionReportIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDataUsagePerPDUSessionReportIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDataUsagePerPDUSessionReportIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDataUsagePerPDUSessionReportIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DataUsagePerPDUSessionReportIEExtensions(
        pub SequenceOf<AnonymousDataUsagePerPDUSessionReportIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Data-Usage-per-PDU-Session-Report")]
    #[non_exhaustive]
    pub struct DataUsagePerPDUSessionReport {
        #[rasn(identifier = "secondaryRATType")]
        pub secondary_rattype: DataUsagePerPDUSessionReportSecondaryRATType,
        #[rasn(size("1..=2"), identifier = "pDU-session-Timed-Report-List")]
        pub p_du_session_timed_report_list: SequenceOf<MRDCDataUsageReportItem>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DataUsagePerPDUSessionReportIEExtensions>,
    }
    impl DataUsagePerPDUSessionReport {
        pub fn new(
            secondary_rattype: DataUsagePerPDUSessionReportSecondaryRATType,
            p_du_session_timed_report_list: SequenceOf<MRDCDataUsageReportItem>,
            i_e_extensions: Option<DataUsagePerPDUSessionReportIEExtensions>,
        ) -> Self {
            Self {
                secondary_rattype,
                p_du_session_timed_report_list,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum DataUsagePerQoSFlowItemSecondaryRATType {
        nR = 0,
        #[rasn(identifier = "e-UTRA")]
        e_UTRA = 1,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDataUsagePerQoSFlowItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDataUsagePerQoSFlowItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDataUsagePerQoSFlowItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDataUsagePerQoSFlowItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDataUsagePerQoSFlowItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DataUsagePerQoSFlowItemIEExtensions(
        pub SequenceOf<AnonymousDataUsagePerQoSFlowItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Data-Usage-per-QoS-Flow-Item")]
    #[non_exhaustive]
    pub struct DataUsagePerQoSFlowItem {
        #[rasn(identifier = "qoS-Flow-Identifier")]
        pub qo_s_flow_identifier: QoSFlowIdentifier,
        #[rasn(identifier = "secondaryRATType")]
        pub secondary_rattype: DataUsagePerQoSFlowItemSecondaryRATType,
        #[rasn(size("1..=2"), identifier = "qoS-Flow-Timed-Report-List")]
        pub qo_s_flow_timed_report_list: SequenceOf<MRDCDataUsageReportItem>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DataUsagePerQoSFlowItemIEExtensions>,
    }
    impl DataUsagePerQoSFlowItem {
        pub fn new(
            qo_s_flow_identifier: QoSFlowIdentifier,
            secondary_rattype: DataUsagePerQoSFlowItemSecondaryRATType,
            qo_s_flow_timed_report_list: SequenceOf<MRDCDataUsageReportItem>,
            i_e_extensions: Option<DataUsagePerQoSFlowItemIEExtensions>,
        ) -> Self {
            Self {
                qo_s_flow_identifier,
                secondary_rattype,
                qo_s_flow_timed_report_list,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=64"), identifier = "Data-Usage-per-QoS-Flow-List")]
    pub struct DataUsagePerQoSFlowList(pub SequenceOf<DataUsagePerQoSFlowItem>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum DataDiscardRequired {
        required = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "DataForwardingtoE-UTRANInformationList"
    )]
    pub struct DataForwardingtoEUTRANInformationList(
        pub SequenceOf<DataForwardingtoEUTRANInformationListItem>,
    );
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDataForwardingtoEUTRANInformationListItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDataForwardingtoEUTRANInformationListItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDataForwardingtoEUTRANInformationListItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDataForwardingtoEUTRANInformationListItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDataForwardingtoEUTRANInformationListItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DataForwardingtoEUTRANInformationListItemIEExtensions(
        pub SequenceOf<AnonymousDataForwardingtoEUTRANInformationListItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "DataForwardingtoE-UTRANInformationListItem"
    )]
    #[non_exhaustive]
    pub struct DataForwardingtoEUTRANInformationListItem {
        #[rasn(identifier = "data-forwarding-tunnel-information")]
        pub data_forwarding_tunnel_information: UPTNLInformation,
        #[rasn(identifier = "qoS-Flows-to-be-forwarded-List")]
        pub qo_s_flows_to_be_forwarded_list: QoSFlowsToBeForwardedList,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DataForwardingtoEUTRANInformationListItemIEExtensions>,
    }
    impl DataForwardingtoEUTRANInformationListItem {
        pub fn new(
            data_forwarding_tunnel_information: UPTNLInformation,
            qo_s_flows_to_be_forwarded_list: QoSFlowsToBeForwardedList,
            i_e_extensions: Option<DataForwardingtoEUTRANInformationListItemIEExtensions>,
        ) -> Self {
            Self {
                data_forwarding_tunnel_information,
                qo_s_flows_to_be_forwarded_list,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=64"),
        identifier = "DataForwardingtoNG-RANQoSFlowInformationList"
    )]
    pub struct DataForwardingtoNGRANQoSFlowInformationList(
        pub SequenceOf<DataForwardingtoNGRANQoSFlowInformationListItem>,
    );
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDataForwardingtoNGRANQoSFlowInformationListItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDataForwardingtoNGRANQoSFlowInformationListItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality:
            AnonymousDataForwardingtoNGRANQoSFlowInformationListItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDataForwardingtoNGRANQoSFlowInformationListItemIEExtensions {
        pub fn new(
            id: u16,
            criticality : AnonymousDataForwardingtoNGRANQoSFlowInformationListItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct DataForwardingtoNGRANQoSFlowInformationListItemIEExtensions(
        pub SequenceOf<AnonymousDataForwardingtoNGRANQoSFlowInformationListItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "DataForwardingtoNG-RANQoSFlowInformationList-Item"
    )]
    #[non_exhaustive]
    pub struct DataForwardingtoNGRANQoSFlowInformationListItem {
        #[rasn(identifier = "qoS-Flow-Identifier")]
        pub qo_s_flow_identifier: QoSFlowIdentifier,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<DataForwardingtoNGRANQoSFlowInformationListItemIEExtensions>,
    }
    impl DataForwardingtoNGRANQoSFlowInformationListItem {
        pub fn new(
            qo_s_flow_identifier: QoSFlowIdentifier,
            i_e_extensions: Option<DataForwardingtoNGRANQoSFlowInformationListItemIEExtensions>,
        ) -> Self {
            Self {
                qo_s_flow_identifier,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum DefaultDRB {
        #[rasn(identifier = "true")]
        R_true = 0,
        #[rasn(identifier = "false")]
        R_false = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum Dictionary {
        #[rasn(identifier = "sip-SDP")]
        sip_SDP = 0,
        operator = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum DirectForwardingPathAvailability {
        #[rasn(identifier = "inter-system-direct-path-available")]
        inter_system_direct_path_available = 0,
        #[rasn(extension_addition, identifier = "intra-system-direct-path-available")]
        intra_system_direct_path_available = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum DiscardTimer {
        ms10 = 0,
        ms20 = 1,
        ms30 = 2,
        ms40 = 3,
        ms50 = 4,
        ms60 = 5,
        ms75 = 6,
        ms100 = 7,
        ms150 = 8,
        ms200 = 9,
        ms250 = 10,
        ms300 = 11,
        ms500 = 12,
        ms750 = 13,
        ms1500 = 14,
        infinity = 15,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum DiscardTimerExtended {
        ms0dot5 = 0,
        ms1 = 1,
        ms2 = 2,
        ms4 = 3,
        ms6 = 4,
        ms8 = 5,
        #[rasn(extension_addition)]
        ms2000 = 6,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "Duplication-Activation")]
    #[non_exhaustive]
    pub enum DuplicationActivation {
        active = 0,
        inactive = 1,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum Dynamic5QIDescriptorDelayCritical {
        #[rasn(identifier = "delay-critical")]
        delay_critical = 0,
        #[rasn(identifier = "non-delay-critical")]
        non_delay_critical = 1,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDynamic5QIDescriptorIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDynamic5QIDescriptorIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDynamic5QIDescriptorIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousDynamic5QIDescriptorIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousDynamic5QIDescriptorIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct Dynamic5QIDescriptorIEExtensions(
        pub SequenceOf<AnonymousDynamic5QIDescriptorIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct Dynamic5QIDescriptor {
        #[rasn(identifier = "qoSPriorityLevel")]
        pub qo_spriority_level: QoSPriorityLevel,
        #[rasn(identifier = "packetDelayBudget")]
        pub packet_delay_budget: PacketDelayBudget,
        #[rasn(identifier = "packetErrorRate")]
        pub packet_error_rate: PacketErrorRate,
        #[rasn(value("0..=255", extensible), identifier = "fiveQI")]
        pub five_qi: Option<Integer>,
        #[rasn(identifier = "delayCritical")]
        pub delay_critical: Option<Dynamic5QIDescriptorDelayCritical>,
        #[rasn(identifier = "averagingWindow")]
        pub averaging_window: Option<AveragingWindow>,
        #[rasn(identifier = "maxDataBurstVolume")]
        pub max_data_burst_volume: Option<MaxDataBurstVolume>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<Dynamic5QIDescriptorIEExtensions>,
    }
    impl Dynamic5QIDescriptor {
        pub fn new(
            qo_spriority_level: QoSPriorityLevel,
            packet_delay_budget: PacketDelayBudget,
            packet_error_rate: PacketErrorRate,
            five_qi: Option<Integer>,
            delay_critical: Option<Dynamic5QIDescriptorDelayCritical>,
            averaging_window: Option<AveragingWindow>,
            max_data_burst_volume: Option<MaxDataBurstVolume>,
            i_e_extensions: Option<Dynamic5QIDescriptorIEExtensions>,
        ) -> Self {
            Self {
                qo_spriority_level,
                packet_delay_budget,
                packet_error_rate,
                five_qi,
                delay_critical,
                averaging_window,
                max_data_burst_volume,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "E-UTRAN-Cell-Identity")]
    pub struct EUTRANCellIdentity(pub FixedBitString<28usize>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousECGIIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousECGIIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousECGIIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousECGIIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousECGIIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct ECGIIEExtensions(pub SequenceOf<AnonymousECGIIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ECGI {
        #[rasn(identifier = "pLMN-Identity")]
        pub p_lmn_identity: PLMNIdentity,
        #[rasn(identifier = "eUTRAN-Cell-Identity")]
        pub e_utran_cell_identity: EUTRANCellIdentity,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ECGIIEExtensions>,
    }
    impl ECGI {
        pub fn new(
            p_lmn_identity: PLMNIdentity,
            e_utran_cell_identity: EUTRANCellIdentity,
            i_e_extensions: Option<ECGIIEExtensions>,
        ) -> Self {
            Self {
                p_lmn_identity,
                e_utran_cell_identity,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousECGISupportItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousECGISupportItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousECGISupportItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousECGISupportItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousECGISupportItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct ECGISupportItemIEExtensions(pub SequenceOf<AnonymousECGISupportItemIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "ECGI-Support-Item")]
    pub struct ECGISupportItem {
        #[rasn(identifier = "eCGI")]
        pub e_cgi: ECGI,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ECGISupportItemIEExtensions>,
    }
    impl ECGISupportItem {
        pub fn new(e_cgi: ECGI, i_e_extensions: Option<ECGISupportItemIEExtensions>) -> Self {
            Self {
                e_cgi,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=512"), identifier = "ECGI-Support-List")]
    pub struct ECGISupportList(pub SequenceOf<ECGISupportItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ECNMarkingorCongestionInformationReportingRequestECNMarkingatNGRAN {
        ul = 0,
        dl = 1,
        both = 2,
        stop = 3,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ECNMarkingorCongestionInformationReportingRequestECNMarkingatUPF {
        ul = 0,
        dl = 1,
        both = 2,
        stop = 3,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ECNMarkingorCongestionInformationReportingRequestCongestionInformation {
        ul = 0,
        dl = 1,
        both = 2,
        stop = 3,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum ECNMarkingorCongestionInformationReportingRequestChoiceExtensionCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ECNMarkingorCongestionInformationReportingRequestChoiceExtension {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality:
            ECNMarkingorCongestionInformationReportingRequestChoiceExtensionCriticality,
        pub value: Any,
    }
    impl ECNMarkingorCongestionInformationReportingRequestChoiceExtension {
        pub fn new(
            id: u16,
            criticality : ECNMarkingorCongestionInformationReportingRequestChoiceExtensionCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum ECNMarkingorCongestionInformationReportingRequest {
        eCNMarkingatNGRAN(ECNMarkingorCongestionInformationReportingRequestECNMarkingatNGRAN),
        eCNMarkingatUPF(ECNMarkingorCongestionInformationReportingRequestECNMarkingatUPF),
        congestionInformation(
            ECNMarkingorCongestionInformationReportingRequestCongestionInformation,
        ),
        #[rasn(identifier = "choice-extension")]
        choice_extension(ECNMarkingorCongestionInformationReportingRequestChoiceExtension),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ECNMarkingorCongestionInformationReportingStatus {
        active = 0,
        #[rasn(identifier = "not-active")]
        not_active = 1,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum EHCCommonParametersEhcCIDLength {
        bits7 = 0,
        bits15 = 1,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousEHCCommonParametersIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousEHCCommonParametersIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousEHCCommonParametersIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousEHCCommonParametersIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousEHCCommonParametersIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct EHCCommonParametersIEExtensions(
        pub SequenceOf<AnonymousEHCCommonParametersIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "EHC-Common-Parameters")]
    pub struct EHCCommonParameters {
        #[rasn(identifier = "ehc-CID-Length")]
        pub ehc_cid_length: EHCCommonParametersEhcCIDLength,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<EHCCommonParametersIEExtensions>,
    }
    impl EHCCommonParameters {
        pub fn new(
            ehc_cid_length: EHCCommonParametersEhcCIDLength,
            i_e_extensions: Option<EHCCommonParametersIEExtensions>,
        ) -> Self {
            Self {
                ehc_cid_length,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum EHCDownlinkParametersDrbContinueEHCDL {
        #[rasn(identifier = "true")]
        R_true = 0,
        #[rasn(extension_addition, identifier = "false")]
        R_false = 1,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousEHCDownlinkParametersIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousEHCDownlinkParametersIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousEHCDownlinkParametersIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousEHCDownlinkParametersIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousEHCDownlinkParametersIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct EHCDownlinkParametersIEExtensions(
        pub SequenceOf<AnonymousEHCDownlinkParametersIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "EHC-Downlink-Parameters")]
    pub struct EHCDownlinkParameters {
        #[rasn(identifier = "drb-ContinueEHC-DL")]
        pub drb_continue_ehc_dl: EHCDownlinkParametersDrbContinueEHCDL,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<EHCDownlinkParametersIEExtensions>,
    }
    impl EHCDownlinkParameters {
        pub fn new(
            drb_continue_ehc_dl: EHCDownlinkParametersDrbContinueEHCDL,
            i_e_extensions: Option<EHCDownlinkParametersIEExtensions>,
        ) -> Self {
            Self {
                drb_continue_ehc_dl,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousEHCParametersIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousEHCParametersIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousEHCParametersIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousEHCParametersIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousEHCParametersIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct EHCParametersIEExtensions(pub SequenceOf<AnonymousEHCParametersIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "EHC-Parameters")]
    pub struct EHCParameters {
        #[rasn(identifier = "ehc-Common")]
        pub ehc_common: EHCCommonParameters,
        #[rasn(identifier = "ehc-Downlink")]
        pub ehc_downlink: Option<EHCDownlinkParameters>,
        #[rasn(identifier = "ehc-Uplink")]
        pub ehc_uplink: Option<EHCUplinkParameters>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<EHCParametersIEExtensions>,
    }
    impl EHCParameters {
        pub fn new(
            ehc_common: EHCCommonParameters,
            ehc_downlink: Option<EHCDownlinkParameters>,
            ehc_uplink: Option<EHCUplinkParameters>,
            i_e_extensions: Option<EHCParametersIEExtensions>,
        ) -> Self {
            Self {
                ehc_common,
                ehc_downlink,
                ehc_uplink,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum EHCUplinkParametersDrbContinueEHCUL {
        #[rasn(identifier = "true")]
        R_true = 0,
        #[rasn(extension_addition, identifier = "false")]
        R_false = 1,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousEHCUplinkParametersIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousEHCUplinkParametersIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousEHCUplinkParametersIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousEHCUplinkParametersIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousEHCUplinkParametersIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct EHCUplinkParametersIEExtensions(
        pub SequenceOf<AnonymousEHCUplinkParametersIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "EHC-Uplink-Parameters")]
    pub struct EHCUplinkParameters {
        #[rasn(identifier = "drb-ContinueEHC-UL")]
        pub drb_continue_ehc_ul: EHCUplinkParametersDrbContinueEHCUL,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<EHCUplinkParametersIEExtensions>,
    }
    impl EHCUplinkParameters {
        pub fn new(
            drb_continue_ehc_ul: EHCUplinkParametersDrbContinueEHCUL,
            i_e_extensions: Option<EHCUplinkParametersIEExtensions>,
        ) -> Self {
            Self {
                drb_continue_ehc_ul,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousEUTRANQoSIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousEUTRANQoSIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousEUTRANQoSIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousEUTRANQoSIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousEUTRANQoSIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct EUTRANQoSIEExtensions(pub SequenceOf<AnonymousEUTRANQoSIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "EUTRAN-QoS")]
    #[non_exhaustive]
    pub struct EUTRANQoS {
        #[rasn(identifier = "qCI")]
        pub q_ci: QCI,
        #[rasn(identifier = "eUTRANallocationAndRetentionPriority")]
        pub e_utranallocation_and_retention_priority: EUTRANAllocationAndRetentionPriority,
        #[rasn(identifier = "gbrQosInformation")]
        pub gbr_qos_information: Option<GBRQosInformation>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<EUTRANQoSIEExtensions>,
    }
    impl EUTRANQoS {
        pub fn new(
            q_ci: QCI,
            e_utranallocation_and_retention_priority: EUTRANAllocationAndRetentionPriority,
            gbr_qos_information: Option<GBRQosInformation>,
            i_e_extensions: Option<EUTRANQoSIEExtensions>,
        ) -> Self {
            Self {
                q_ci,
                e_utranallocation_and_retention_priority,
                gbr_qos_information,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousEUTRANQoSSupportItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousEUTRANQoSSupportItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousEUTRANQoSSupportItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousEUTRANQoSSupportItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousEUTRANQoSSupportItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct EUTRANQoSSupportItemIEExtensions(
        pub SequenceOf<AnonymousEUTRANQoSSupportItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "EUTRAN-QoS-Support-Item")]
    pub struct EUTRANQoSSupportItem {
        #[rasn(identifier = "eUTRAN-QoS")]
        pub e_utran_qo_s: EUTRANQoS,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<EUTRANQoSSupportItemIEExtensions>,
    }
    impl EUTRANQoSSupportItem {
        pub fn new(
            e_utran_qo_s: EUTRANQoS,
            i_e_extensions: Option<EUTRANQoSSupportItemIEExtensions>,
        ) -> Self {
            Self {
                e_utran_qo_s,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"), identifier = "EUTRAN-QoS-Support-List")]
    pub struct EUTRANQoSSupportList(pub SequenceOf<EUTRANQoSSupportItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousEUTRANAllocationAndRetentionPriorityIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousEUTRANAllocationAndRetentionPriorityIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousEUTRANAllocationAndRetentionPriorityIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousEUTRANAllocationAndRetentionPriorityIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousEUTRANAllocationAndRetentionPriorityIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct EUTRANAllocationAndRetentionPriorityIEExtensions(
        pub SequenceOf<AnonymousEUTRANAllocationAndRetentionPriorityIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct EUTRANAllocationAndRetentionPriority {
        #[rasn(identifier = "priorityLevel")]
        pub priority_level: PriorityLevel,
        #[rasn(identifier = "pre-emptionCapability")]
        pub pre_emption_capability: PreEmptionCapability,
        #[rasn(identifier = "pre-emptionVulnerability")]
        pub pre_emption_vulnerability: PreEmptionVulnerability,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<EUTRANAllocationAndRetentionPriorityIEExtensions>,
    }
    impl EUTRANAllocationAndRetentionPriority {
        pub fn new(
            priority_level: PriorityLevel,
            pre_emption_capability: PreEmptionCapability,
            pre_emption_vulnerability: PreEmptionVulnerability,
            i_e_extensions: Option<EUTRANAllocationAndRetentionPriorityIEExtensions>,
        ) -> Self {
            Self {
                priority_level,
                pre_emption_capability,
                pre_emption_vulnerability,
                i_e_extensions,
            }
        }
    }
    #[doc = " E"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum EarlyDataForwardingIndicator {
        stop = 0,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum EarlyForwardingCOUNTInfoChoiceExtensionCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct EarlyForwardingCOUNTInfoChoiceExtension {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: EarlyForwardingCOUNTInfoChoiceExtensionCriticality,
        pub value: Any,
    }
    impl EarlyForwardingCOUNTInfoChoiceExtension {
        pub fn new(
            id: u16,
            criticality: EarlyForwardingCOUNTInfoChoiceExtensionCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum EarlyForwardingCOUNTInfo {
        firstDLCount(FirstDLCount),
        dLDiscardingCount(DLDiscarding),
        #[rasn(identifier = "choice-Extension")]
        choice_Extension(EarlyForwardingCOUNTInfoChoiceExtension),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum EarlyForwardingCOUNTReq {
        #[rasn(identifier = "first-dl-count")]
        first_dl_count = 0,
        #[rasn(identifier = "dl-discarding")]
        dl_discarding = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct EncryptionKey(pub OctetString);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousEndpointIPAddressAndPortIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousEndpointIPAddressAndPortIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousEndpointIPAddressAndPortIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousEndpointIPAddressAndPortIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousEndpointIPAddressAndPortIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct EndpointIPAddressAndPortIEExtensions(
        pub SequenceOf<AnonymousEndpointIPAddressAndPortIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Endpoint-IP-address-and-port")]
    pub struct EndpointIPAddressAndPort {
        #[rasn(identifier = "endpoint-IP-Address")]
        pub endpoint_ip_address: TransportLayerAddress,
        #[rasn(identifier = "portNumber")]
        pub port_number: PortNumber,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<EndpointIPAddressAndPortIEExtensions>,
    }
    impl EndpointIPAddressAndPort {
        pub fn new(
            endpoint_ip_address: TransportLayerAddress,
            port_number: PortNumber,
            i_e_extensions: Option<EndpointIPAddressAndPortIEExtensions>,
        ) -> Self {
            Self {
                endpoint_ip_address,
                port_number,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousExtendedGNBCUCPNameIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousExtendedGNBCUCPNameIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousExtendedGNBCUCPNameIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousExtendedGNBCUCPNameIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousExtendedGNBCUCPNameIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct ExtendedGNBCUCPNameIEExtensions(
        pub SequenceOf<AnonymousExtendedGNBCUCPNameIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Extended-GNB-CU-CP-Name")]
    #[non_exhaustive]
    pub struct ExtendedGNBCUCPName {
        #[rasn(identifier = "gNB-CU-CP-NameVisibleString")]
        pub g_nb_cu_cp_name_visible_string: Option<GNBCUCPNameVisibleString>,
        #[rasn(identifier = "gNB-CU-CP-NameUTF8String")]
        pub g_nb_cu_cp_name_utf8_string: Option<GNBCUCPNameUTF8String>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ExtendedGNBCUCPNameIEExtensions>,
    }
    impl ExtendedGNBCUCPName {
        pub fn new(
            g_nb_cu_cp_name_visible_string: Option<GNBCUCPNameVisibleString>,
            g_nb_cu_cp_name_utf8_string: Option<GNBCUCPNameUTF8String>,
            i_e_extensions: Option<ExtendedGNBCUCPNameIEExtensions>,
        ) -> Self {
            Self {
                g_nb_cu_cp_name_visible_string,
                g_nb_cu_cp_name_utf8_string,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousExtendedGNBCUUPNameIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousExtendedGNBCUUPNameIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousExtendedGNBCUUPNameIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousExtendedGNBCUUPNameIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousExtendedGNBCUUPNameIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct ExtendedGNBCUUPNameIEExtensions(
        pub SequenceOf<AnonymousExtendedGNBCUUPNameIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Extended-GNB-CU-UP-Name")]
    #[non_exhaustive]
    pub struct ExtendedGNBCUUPName {
        #[rasn(identifier = "gNB-CU-UP-NameVisibleString")]
        pub g_nb_cu_up_name_visible_string: Option<GNBCUUPNameVisibleString>,
        #[rasn(identifier = "gNB-CU-UP-NameUTF8String")]
        pub g_nb_cu_up_name_utf8_string: Option<GNBCUUPNameUTF8String>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ExtendedGNBCUUPNameIEExtensions>,
    }
    impl ExtendedGNBCUUPName {
        pub fn new(
            g_nb_cu_up_name_visible_string: Option<GNBCUUPNameVisibleString>,
            g_nb_cu_up_name_utf8_string: Option<GNBCUUPNameUTF8String>,
            i_e_extensions: Option<ExtendedGNBCUUPNameIEExtensions>,
        ) -> Self {
            Self {
                g_nb_cu_up_name_visible_string,
                g_nb_cu_up_name_utf8_string,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousExtendedNRCGISupportItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousExtendedNRCGISupportItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousExtendedNRCGISupportItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousExtendedNRCGISupportItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousExtendedNRCGISupportItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct ExtendedNRCGISupportItemIEExtensions(
        pub SequenceOf<AnonymousExtendedNRCGISupportItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Extended-NR-CGI-Support-Item")]
    pub struct ExtendedNRCGISupportItem {
        #[rasn(identifier = "nR-CGI")]
        pub n_r_cgi: NRCGI,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ExtendedNRCGISupportItemIEExtensions>,
    }
    impl ExtendedNRCGISupportItem {
        pub fn new(
            n_r_cgi: NRCGI,
            i_e_extensions: Option<ExtendedNRCGISupportItemIEExtensions>,
        ) -> Self {
            Self {
                n_r_cgi,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=16384"),
        identifier = "Extended-NR-CGI-Support-List"
    )]
    pub struct ExtendedNRCGISupportList(pub SequenceOf<ExtendedNRCGISupportItem>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=65535", extensible))]
    pub struct ExtendedPacketDelayBudget(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct ExtendedSliceSupportList(pub SequenceOf<SliceSupportItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousF1UTNLInfoAddedItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousF1UTNLInfoAddedItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousF1UTNLInfoAddedItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousF1UTNLInfoAddedItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousF1UTNLInfoAddedItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct F1UTNLInfoAddedItemIEExtensions(
        pub SequenceOf<AnonymousF1UTNLInfoAddedItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "F1U-TNL-InfoAdded-Item")]
    #[non_exhaustive]
    pub struct F1UTNLInfoAddedItem {
        #[rasn(identifier = "broadcastF1U-ContextReferenceE1")]
        pub broadcast_f1_u_context_reference_e1: BroadcastF1UContextReferenceE1,
        #[rasn(identifier = "bcBearerContextF1U-TNLInfoatCU")]
        pub bc_bearer_context_f1_u_tnlinfoat_cu: BCBearerContextF1UTNLInfoatCU,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<F1UTNLInfoAddedItemIEExtensions>,
    }
    impl F1UTNLInfoAddedItem {
        pub fn new(
            broadcast_f1_u_context_reference_e1: BroadcastF1UContextReferenceE1,
            bc_bearer_context_f1_u_tnlinfoat_cu: BCBearerContextF1UTNLInfoatCU,
            i_e_extensions: Option<F1UTNLInfoAddedItemIEExtensions>,
        ) -> Self {
            Self {
                broadcast_f1_u_context_reference_e1,
                bc_bearer_context_f1_u_tnlinfoat_cu,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=512"), identifier = "F1U-TNL-InfoAdded-List")]
    pub struct F1UTNLInfoAddedList(pub SequenceOf<F1UTNLInfoAddedItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousF1UTNLInfoAddedOrModifiedItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousF1UTNLInfoAddedOrModifiedItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousF1UTNLInfoAddedOrModifiedItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousF1UTNLInfoAddedOrModifiedItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousF1UTNLInfoAddedOrModifiedItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct F1UTNLInfoAddedOrModifiedItemIEExtensions(
        pub SequenceOf<AnonymousF1UTNLInfoAddedOrModifiedItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "F1U-TNL-InfoAddedOrModified-Item")]
    #[non_exhaustive]
    pub struct F1UTNLInfoAddedOrModifiedItem {
        #[rasn(identifier = "broadcastF1U-ContextReferenceE1")]
        pub broadcast_f1_u_context_reference_e1: BroadcastF1UContextReferenceE1,
        #[rasn(identifier = "bcBearerContextF1U-TNLInfoatCU")]
        pub bc_bearer_context_f1_u_tnlinfoat_cu: Option<BCBearerContextF1UTNLInfoatCU>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<F1UTNLInfoAddedOrModifiedItemIEExtensions>,
    }
    impl F1UTNLInfoAddedOrModifiedItem {
        pub fn new(
            broadcast_f1_u_context_reference_e1: BroadcastF1UContextReferenceE1,
            bc_bearer_context_f1_u_tnlinfoat_cu: Option<BCBearerContextF1UTNLInfoatCU>,
            i_e_extensions: Option<F1UTNLInfoAddedOrModifiedItemIEExtensions>,
        ) -> Self {
            Self {
                broadcast_f1_u_context_reference_e1,
                bc_bearer_context_f1_u_tnlinfoat_cu,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=512"),
        identifier = "F1U-TNL-InfoAddedOrModified-List"
    )]
    pub struct F1UTNLInfoAddedOrModifiedList(pub SequenceOf<F1UTNLInfoAddedOrModifiedItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousF1UTNLInfoToAddItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousF1UTNLInfoToAddItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousF1UTNLInfoToAddItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousF1UTNLInfoToAddItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousF1UTNLInfoToAddItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct F1UTNLInfoToAddItemIEExtensions(
        pub SequenceOf<AnonymousF1UTNLInfoToAddItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "F1U-TNL-InfoToAdd-Item")]
    #[non_exhaustive]
    pub struct F1UTNLInfoToAddItem {
        #[rasn(identifier = "broadcastF1U-ContextReferenceE1")]
        pub broadcast_f1_u_context_reference_e1: BroadcastF1UContextReferenceE1,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<F1UTNLInfoToAddItemIEExtensions>,
    }
    impl F1UTNLInfoToAddItem {
        pub fn new(
            broadcast_f1_u_context_reference_e1: BroadcastF1UContextReferenceE1,
            i_e_extensions: Option<F1UTNLInfoToAddItemIEExtensions>,
        ) -> Self {
            Self {
                broadcast_f1_u_context_reference_e1,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=512"), identifier = "F1U-TNL-InfoToAdd-List")]
    pub struct F1UTNLInfoToAddList(pub SequenceOf<F1UTNLInfoToAddItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousF1UTNLInfoToAddOrModifyItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousF1UTNLInfoToAddOrModifyItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousF1UTNLInfoToAddOrModifyItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousF1UTNLInfoToAddOrModifyItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousF1UTNLInfoToAddOrModifyItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct F1UTNLInfoToAddOrModifyItemIEExtensions(
        pub SequenceOf<AnonymousF1UTNLInfoToAddOrModifyItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "F1U-TNL-InfoToAddOrModify-Item")]
    #[non_exhaustive]
    pub struct F1UTNLInfoToAddOrModifyItem {
        #[rasn(identifier = "broadcastF1U-ContextReferenceE1")]
        pub broadcast_f1_u_context_reference_e1: BroadcastF1UContextReferenceE1,
        #[rasn(identifier = "bcBearerContextF1U-TNLInfoatDU")]
        pub bc_bearer_context_f1_u_tnlinfoat_du: Option<BCBearerContextF1UTNLInfoatDU>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<F1UTNLInfoToAddOrModifyItemIEExtensions>,
    }
    impl F1UTNLInfoToAddOrModifyItem {
        pub fn new(
            broadcast_f1_u_context_reference_e1: BroadcastF1UContextReferenceE1,
            bc_bearer_context_f1_u_tnlinfoat_du: Option<BCBearerContextF1UTNLInfoatDU>,
            i_e_extensions: Option<F1UTNLInfoToAddOrModifyItemIEExtensions>,
        ) -> Self {
            Self {
                broadcast_f1_u_context_reference_e1,
                bc_bearer_context_f1_u_tnlinfoat_du,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=512"),
        identifier = "F1U-TNL-InfoToAddOrModify-List"
    )]
    pub struct F1UTNLInfoToAddOrModifyList(pub SequenceOf<F1UTNLInfoToAddOrModifyItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousF1UTNLInfoToReleaseItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousF1UTNLInfoToReleaseItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousF1UTNLInfoToReleaseItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousF1UTNLInfoToReleaseItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousF1UTNLInfoToReleaseItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct F1UTNLInfoToReleaseItemIEExtensions(
        pub SequenceOf<AnonymousF1UTNLInfoToReleaseItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "F1U-TNL-InfoToRelease-Item")]
    #[non_exhaustive]
    pub struct F1UTNLInfoToReleaseItem {
        #[rasn(identifier = "broadcastF1U-ContextReferenceE1")]
        pub broadcast_f1_u_context_reference_e1: BroadcastF1UContextReferenceE1,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<F1UTNLInfoToReleaseItemIEExtensions>,
    }
    impl F1UTNLInfoToReleaseItem {
        pub fn new(
            broadcast_f1_u_context_reference_e1: BroadcastF1UContextReferenceE1,
            i_e_extensions: Option<F1UTNLInfoToReleaseItemIEExtensions>,
        ) -> Self {
            Self {
                broadcast_f1_u_context_reference_e1,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=512"), identifier = "F1U-TNL-InfoToRelease-List")]
    pub struct F1UTNLInfoToReleaseList(pub SequenceOf<F1UTNLInfoToReleaseItem>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum F1UTunnelNotEstablished {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousFirstDLCountIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousFirstDLCountIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousFirstDLCountIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousFirstDLCountIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousFirstDLCountIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct FirstDLCountIEExtensions(pub SequenceOf<AnonymousFirstDLCountIEExtensions>);
    #[doc = " F"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct FirstDLCount {
        #[rasn(identifier = "firstDLCountVal")]
        pub first_dlcount_val: PDCPCount,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<FirstDLCountIEExtensions>,
    }
    impl FirstDLCount {
        pub fn new(
            first_dlcount_val: PDCPCount,
            i_e_extensions: Option<FirstDLCountIEExtensions>,
        ) -> Self {
            Self {
                first_dlcount_val,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "FiveGS-TAC")]
    pub struct FiveGSTAC(pub FixedOctetString<3usize>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGBRQoSFlowInformationIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGBRQoSFlowInformationIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGBRQoSFlowInformationIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousGBRQoSFlowInformationIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousGBRQoSFlowInformationIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct GBRQoSFlowInformationIEExtensions(
        pub SequenceOf<AnonymousGBRQoSFlowInformationIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GBR-QoSFlowInformation")]
    #[non_exhaustive]
    pub struct GBRQoSFlowInformation {
        #[rasn(identifier = "maxFlowBitRateDownlink")]
        pub max_flow_bit_rate_downlink: BitRate,
        #[rasn(identifier = "maxFlowBitRateUplink")]
        pub max_flow_bit_rate_uplink: BitRate,
        #[rasn(identifier = "guaranteedFlowBitRateDownlink")]
        pub guaranteed_flow_bit_rate_downlink: BitRate,
        #[rasn(identifier = "guaranteedFlowBitRateUplink")]
        pub guaranteed_flow_bit_rate_uplink: BitRate,
        #[rasn(identifier = "maxPacketLossRateDownlink")]
        pub max_packet_loss_rate_downlink: Option<MaxPacketLossRate>,
        #[rasn(identifier = "maxPacketLossRateUplink")]
        pub max_packet_loss_rate_uplink: Option<MaxPacketLossRate>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<GBRQoSFlowInformationIEExtensions>,
    }
    impl GBRQoSFlowInformation {
        pub fn new(
            max_flow_bit_rate_downlink: BitRate,
            max_flow_bit_rate_uplink: BitRate,
            guaranteed_flow_bit_rate_downlink: BitRate,
            guaranteed_flow_bit_rate_uplink: BitRate,
            max_packet_loss_rate_downlink: Option<MaxPacketLossRate>,
            max_packet_loss_rate_uplink: Option<MaxPacketLossRate>,
            i_e_extensions: Option<GBRQoSFlowInformationIEExtensions>,
        ) -> Self {
            Self {
                max_flow_bit_rate_downlink,
                max_flow_bit_rate_uplink,
                guaranteed_flow_bit_rate_downlink,
                guaranteed_flow_bit_rate_uplink,
                max_packet_loss_rate_downlink,
                max_packet_loss_rate_uplink,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGBRQosInformationIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGBRQosInformationIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGBRQosInformationIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousGBRQosInformationIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousGBRQosInformationIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct GBRQosInformationIEExtensions(
        pub SequenceOf<AnonymousGBRQosInformationIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GBR-QosInformation")]
    #[non_exhaustive]
    pub struct GBRQosInformation {
        #[rasn(identifier = "e-RAB-MaximumBitrateDL")]
        pub e_rab_maximum_bitrate_dl: BitRate,
        #[rasn(identifier = "e-RAB-MaximumBitrateUL")]
        pub e_rab_maximum_bitrate_ul: BitRate,
        #[rasn(identifier = "e-RAB-GuaranteedBitrateDL")]
        pub e_rab_guaranteed_bitrate_dl: BitRate,
        #[rasn(identifier = "e-RAB-GuaranteedBitrateUL")]
        pub e_rab_guaranteed_bitrate_ul: BitRate,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<GBRQosInformationIEExtensions>,
    }
    impl GBRQosInformation {
        pub fn new(
            e_rab_maximum_bitrate_dl: BitRate,
            e_rab_maximum_bitrate_ul: BitRate,
            e_rab_guaranteed_bitrate_dl: BitRate,
            e_rab_guaranteed_bitrate_ul: BitRate,
            i_e_extensions: Option<GBRQosInformationIEExtensions>,
        ) -> Self {
            Self {
                e_rab_maximum_bitrate_dl,
                e_rab_maximum_bitrate_ul,
                e_rab_guaranteed_bitrate_dl,
                e_rab_guaranteed_bitrate_ul,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "GNB-CU-CP-MBS-E1AP-ID", value("0..=16777215"))]
    pub struct GNBCUCPMBSE1APID(pub u32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "GNB-CU-CP-Name", size("1..=150", extensible))]
    pub struct GNBCUCPName(pub PrintableString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "GNB-CU-CP-NameUTF8String",
        size("1..=150", extensible)
    )]
    pub struct GNBCUCPNameUTF8String(pub Utf8String);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "GNB-CU-CP-NameVisibleString",
        size("1..=150", extensible)
    )]
    pub struct GNBCUCPNameVisibleString(pub VisibleString);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGNBCUCPTNLAFailedToSetupItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGNBCUCPTNLAFailedToSetupItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGNBCUCPTNLAFailedToSetupItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousGNBCUCPTNLAFailedToSetupItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousGNBCUCPTNLAFailedToSetupItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct GNBCUCPTNLAFailedToSetupItemIEExtensions(
        pub SequenceOf<AnonymousGNBCUCPTNLAFailedToSetupItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GNB-CU-CP-TNLA-Failed-To-Setup-Item")]
    pub struct GNBCUCPTNLAFailedToSetupItem {
        #[rasn(identifier = "tNLAssociationTransportLayerAddress")]
        pub t_nlassociation_transport_layer_address: CPTNLInformation,
        pub cause: Cause,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<GNBCUCPTNLAFailedToSetupItemIEExtensions>,
    }
    impl GNBCUCPTNLAFailedToSetupItem {
        pub fn new(
            t_nlassociation_transport_layer_address: CPTNLInformation,
            cause: Cause,
            i_e_extensions: Option<GNBCUCPTNLAFailedToSetupItemIEExtensions>,
        ) -> Self {
            Self {
                t_nlassociation_transport_layer_address,
                cause,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGNBCUCPTNLASetupItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGNBCUCPTNLASetupItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGNBCUCPTNLASetupItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousGNBCUCPTNLASetupItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousGNBCUCPTNLASetupItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct GNBCUCPTNLASetupItemIEExtensions(
        pub SequenceOf<AnonymousGNBCUCPTNLASetupItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GNB-CU-CP-TNLA-Setup-Item")]
    #[non_exhaustive]
    pub struct GNBCUCPTNLASetupItem {
        #[rasn(identifier = "tNLAssociationTransportLayerAddress")]
        pub t_nlassociation_transport_layer_address: CPTNLInformation,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<GNBCUCPTNLASetupItemIEExtensions>,
    }
    impl GNBCUCPTNLASetupItem {
        pub fn new(
            t_nlassociation_transport_layer_address: CPTNLInformation,
            i_e_extensions: Option<GNBCUCPTNLASetupItemIEExtensions>,
        ) -> Self {
            Self {
                t_nlassociation_transport_layer_address,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGNBCUCPTNLAToAddItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGNBCUCPTNLAToAddItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGNBCUCPTNLAToAddItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousGNBCUCPTNLAToAddItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousGNBCUCPTNLAToAddItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct GNBCUCPTNLAToAddItemIEExtensions(
        pub SequenceOf<AnonymousGNBCUCPTNLAToAddItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GNB-CU-CP-TNLA-To-Add-Item")]
    pub struct GNBCUCPTNLAToAddItem {
        #[rasn(identifier = "tNLAssociationTransportLayerAddress")]
        pub t_nlassociation_transport_layer_address: CPTNLInformation,
        #[rasn(identifier = "tNLAssociationUsage")]
        pub t_nlassociation_usage: TNLAssociationUsage,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<GNBCUCPTNLAToAddItemIEExtensions>,
    }
    impl GNBCUCPTNLAToAddItem {
        pub fn new(
            t_nlassociation_transport_layer_address: CPTNLInformation,
            t_nlassociation_usage: TNLAssociationUsage,
            i_e_extensions: Option<GNBCUCPTNLAToAddItemIEExtensions>,
        ) -> Self {
            Self {
                t_nlassociation_transport_layer_address,
                t_nlassociation_usage,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGNBCUCPTNLAToRemoveItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGNBCUCPTNLAToRemoveItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGNBCUCPTNLAToRemoveItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousGNBCUCPTNLAToRemoveItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousGNBCUCPTNLAToRemoveItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct GNBCUCPTNLAToRemoveItemIEExtensions(
        pub SequenceOf<AnonymousGNBCUCPTNLAToRemoveItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GNB-CU-CP-TNLA-To-Remove-Item")]
    pub struct GNBCUCPTNLAToRemoveItem {
        #[rasn(identifier = "tNLAssociationTransportLayerAddress")]
        pub t_nlassociation_transport_layer_address: CPTNLInformation,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<GNBCUCPTNLAToRemoveItemIEExtensions>,
    }
    impl GNBCUCPTNLAToRemoveItem {
        pub fn new(
            t_nlassociation_transport_layer_address: CPTNLInformation,
            i_e_extensions: Option<GNBCUCPTNLAToRemoveItemIEExtensions>,
        ) -> Self {
            Self {
                t_nlassociation_transport_layer_address,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGNBCUCPTNLAToUpdateItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGNBCUCPTNLAToUpdateItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGNBCUCPTNLAToUpdateItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousGNBCUCPTNLAToUpdateItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousGNBCUCPTNLAToUpdateItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct GNBCUCPTNLAToUpdateItemIEExtensions(
        pub SequenceOf<AnonymousGNBCUCPTNLAToUpdateItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GNB-CU-CP-TNLA-To-Update-Item")]
    pub struct GNBCUCPTNLAToUpdateItem {
        #[rasn(identifier = "tNLAssociationTransportLayerAddress")]
        pub t_nlassociation_transport_layer_address: CPTNLInformation,
        #[rasn(identifier = "tNLAssociationUsage")]
        pub t_nlassociation_usage: Option<TNLAssociationUsage>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<GNBCUCPTNLAToUpdateItemIEExtensions>,
    }
    impl GNBCUCPTNLAToUpdateItem {
        pub fn new(
            t_nlassociation_transport_layer_address: CPTNLInformation,
            t_nlassociation_usage: Option<TNLAssociationUsage>,
            i_e_extensions: Option<GNBCUCPTNLAToUpdateItemIEExtensions>,
        ) -> Self {
            Self {
                t_nlassociation_transport_layer_address,
                t_nlassociation_usage,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "GNB-CU-CP-UE-E1AP-ID", value("0..=4294967295"))]
    pub struct GNBCUCPUEE1APID(pub u32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "GNB-CU-UP-Capacity", value("0..=255"))]
    pub struct GNBCUUPCapacity(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=8"),
        identifier = "GNB-CU-UP-CellGroupRelatedConfiguration"
    )]
    pub struct GNBCUUPCellGroupRelatedConfiguration(
        pub SequenceOf<GNBCUUPCellGroupRelatedConfigurationItem>,
    );
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGNBCUUPCellGroupRelatedConfigurationItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGNBCUUPCellGroupRelatedConfigurationItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGNBCUUPCellGroupRelatedConfigurationItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousGNBCUUPCellGroupRelatedConfigurationItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousGNBCUUPCellGroupRelatedConfigurationItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct GNBCUUPCellGroupRelatedConfigurationItemIEExtensions(
        pub SequenceOf<AnonymousGNBCUUPCellGroupRelatedConfigurationItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "GNB-CU-UP-CellGroupRelatedConfiguration-Item"
    )]
    pub struct GNBCUUPCellGroupRelatedConfigurationItem {
        #[rasn(identifier = "cell-Group-ID")]
        pub cell_group_id: CellGroupID,
        #[rasn(identifier = "uP-TNL-Information")]
        pub u_p_tnl_information: UPTNLInformation,
        #[rasn(identifier = "uL-Configuration")]
        pub u_l_configuration: Option<ULConfiguration>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<GNBCUUPCellGroupRelatedConfigurationItemIEExtensions>,
    }
    impl GNBCUUPCellGroupRelatedConfigurationItem {
        pub fn new(
            cell_group_id: CellGroupID,
            u_p_tnl_information: UPTNLInformation,
            u_l_configuration: Option<ULConfiguration>,
            i_e_extensions: Option<GNBCUUPCellGroupRelatedConfigurationItemIEExtensions>,
        ) -> Self {
            Self {
                cell_group_id,
                u_p_tnl_information,
                u_l_configuration,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "GNB-CU-UP-ID", value("0..=68719476735"))]
    pub struct GNBCUUPID(pub u64);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "GNB-CU-UP-MBS-E1AP-ID", value("0..=65535"))]
    pub struct GNBCUUPMBSE1APID(pub u16);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGNBCUUPMBSSupportInfoIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGNBCUUPMBSSupportInfoIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGNBCUUPMBSSupportInfoIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousGNBCUUPMBSSupportInfoIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousGNBCUUPMBSSupportInfoIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct GNBCUUPMBSSupportInfoIEExtensions(
        pub SequenceOf<AnonymousGNBCUUPMBSSupportInfoIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GNB-CU-UP-MBS-Support-Info")]
    #[non_exhaustive]
    pub struct GNBCUUPMBSSupportInfo {
        #[rasn(identifier = "mbs-Support-Info-ToAdd-List")]
        pub mbs_support_info_to_add_list: Option<MBSSupportInfoToAddList>,
        #[rasn(identifier = "mbs-Support-Info-ToRemove-List")]
        pub mbs_support_info_to_remove_list: Option<MBSSupportInfoToRemoveList>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<GNBCUUPMBSSupportInfoIEExtensions>,
    }
    impl GNBCUUPMBSSupportInfo {
        pub fn new(
            mbs_support_info_to_add_list: Option<MBSSupportInfoToAddList>,
            mbs_support_info_to_remove_list: Option<MBSSupportInfoToRemoveList>,
            i_e_extensions: Option<GNBCUUPMBSSupportInfoIEExtensions>,
        ) -> Self {
            Self {
                mbs_support_info_to_add_list,
                mbs_support_info_to_remove_list,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "GNB-CU-UP-Name", size("1..=150", extensible))]
    pub struct GNBCUUPName(pub PrintableString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "GNB-CU-UP-NameUTF8String",
        size("1..=150", extensible)
    )]
    pub struct GNBCUUPNameUTF8String(pub Utf8String);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "GNB-CU-UP-NameVisibleString",
        size("1..=150", extensible)
    )]
    pub struct GNBCUUPNameVisibleString(pub VisibleString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "GNB-CU-UP-OverloadInformation")]
    pub enum GNBCUUPOverloadInformation {
        overloaded = 0,
        #[rasn(identifier = "not-overloaded")]
        not_overloaded = 1,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGNBCUUPTNLAToRemoveItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGNBCUUPTNLAToRemoveItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGNBCUUPTNLAToRemoveItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousGNBCUUPTNLAToRemoveItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousGNBCUUPTNLAToRemoveItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct GNBCUUPTNLAToRemoveItemIEExtensions(
        pub SequenceOf<AnonymousGNBCUUPTNLAToRemoveItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GNB-CU-UP-TNLA-To-Remove-Item")]
    pub struct GNBCUUPTNLAToRemoveItem {
        #[rasn(identifier = "tNLAssociationTransportLayerAddress")]
        pub t_nlassociation_transport_layer_address: CPTNLInformation,
        #[rasn(identifier = "tNLAssociationTransportLayerAddressgNBCUCP")]
        pub t_nlassociation_transport_layer_addressg_nbcucp: Option<CPTNLInformation>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<GNBCUUPTNLAToRemoveItemIEExtensions>,
    }
    impl GNBCUUPTNLAToRemoveItem {
        pub fn new(
            t_nlassociation_transport_layer_address: CPTNLInformation,
            t_nlassociation_transport_layer_addressg_nbcucp: Option<CPTNLInformation>,
            i_e_extensions: Option<GNBCUUPTNLAToRemoveItemIEExtensions>,
        ) -> Self {
            Self {
                t_nlassociation_transport_layer_address,
                t_nlassociation_transport_layer_addressg_nbcucp,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "GNB-CU-UP-UE-E1AP-ID", value("0..=4294967295"))]
    pub struct GNBCUUPUEE1APID(pub u32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "GNB-DU-ID", value("0..=68719476735"))]
    pub struct GNBDUID(pub u64);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "GTP-TEID")]
    pub struct GTPTEID(pub FixedOctetString<4usize>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGTPTLAItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGTPTLAItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGTPTLAItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousGTPTLAItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousGTPTLAItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct GTPTLAItemIEExtensions(pub SequenceOf<AnonymousGTPTLAItemIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GTPTLA-Item")]
    #[non_exhaustive]
    pub struct GTPTLAItem {
        #[rasn(identifier = "gTPTransportLayerAddresses")]
        pub g_tptransport_layer_addresses: TransportLayerAddress,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<GTPTLAItemIEExtensions>,
    }
    impl GTPTLAItem {
        pub fn new(
            g_tptransport_layer_addresses: TransportLayerAddress,
            i_e_extensions: Option<GTPTLAItemIEExtensions>,
        ) -> Self {
            Self {
                g_tptransport_layer_addresses,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct GTPTLAs(pub SequenceOf<GTPTLAItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGTPTunnelIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGTPTunnelIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGTPTunnelIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousGTPTunnelIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousGTPTunnelIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct GTPTunnelIEExtensions(pub SequenceOf<AnonymousGTPTunnelIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct GTPTunnel {
        #[rasn(identifier = "transportLayerAddress")]
        pub transport_layer_address: TransportLayerAddress,
        #[rasn(identifier = "gTP-TEID")]
        pub g_tp_teid: GTPTEID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<GTPTunnelIEExtensions>,
    }
    impl GTPTunnel {
        pub fn new(
            transport_layer_address: TransportLayerAddress,
            g_tp_teid: GTPTEID,
            i_e_extensions: Option<GTPTunnelIEExtensions>,
        ) -> Self {
            Self {
                transport_layer_address,
                g_tp_teid,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGlobalMBSSessionIDIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGlobalMBSSessionIDIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGlobalMBSSessionIDIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousGlobalMBSSessionIDIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousGlobalMBSSessionIDIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct GlobalMBSSessionIDIEExtensions(
        pub SequenceOf<AnonymousGlobalMBSSessionIDIEExtensions>,
    );
    #[doc = " G"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct GlobalMBSSessionID {
        #[rasn(size("6"))]
        pub tmgi: OctetString,
        pub nid: Option<NID>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<GlobalMBSSessionIDIEExtensions>,
    }
    impl GlobalMBSSessionID {
        pub fn new(
            tmgi: OctetString,
            nid: Option<NID>,
            i_e_extensions: Option<GlobalMBSSessionIDIEExtensions>,
        ) -> Self {
            Self {
                tmgi,
                nid,
                i_e_extensions,
            }
        }
    }
    #[doc = " H"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=4294967295"))]
    pub struct HFN(pub u32);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousHWCapacityIndicatorIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousHWCapacityIndicatorIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousHWCapacityIndicatorIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousHWCapacityIndicatorIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousHWCapacityIndicatorIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct HWCapacityIndicatorIEExtensions(
        pub SequenceOf<AnonymousHWCapacityIndicatorIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "HW-CapacityIndicator")]
    #[non_exhaustive]
    pub struct HWCapacityIndicator {
        #[rasn(value("1..=16777216", extensible), identifier = "offeredThroughput")]
        pub offered_throughput: Integer,
        #[rasn(value("0..=100", extensible), identifier = "availableThroughput")]
        pub available_throughput: Integer,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<HWCapacityIndicatorIEExtensions>,
    }
    impl HWCapacityIndicator {
        pub fn new(
            offered_throughput: Integer,
            available_throughput: Integer,
            i_e_extensions: Option<HWCapacityIndicatorIEExtensions>,
        ) -> Self {
            Self {
                offered_throughput,
                available_throughput,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousIABDonorCUUPPSKInfoItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousIABDonorCUUPPSKInfoItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousIABDonorCUUPPSKInfoItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousIABDonorCUUPPSKInfoItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousIABDonorCUUPPSKInfoItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct IABDonorCUUPPSKInfoItemIEExtensions(
        pub SequenceOf<AnonymousIABDonorCUUPPSKInfoItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "IAB-Donor-CU-UPPSKInfo-Item")]
    #[non_exhaustive]
    pub struct IABDonorCUUPPSKInfoItem {
        #[rasn(identifier = "iAB-donor-CU-UPPSK")]
        pub i_ab_donor_cu_uppsk: IABDonorCUUPPSK,
        #[rasn(identifier = "iAB-donor-CU-UPIPAddress")]
        pub i_ab_donor_cu_upipaddress: TransportLayerAddress,
        #[rasn(identifier = "iAB-DUIPAddress")]
        pub i_ab_duipaddress: TransportLayerAddress,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<IABDonorCUUPPSKInfoItemIEExtensions>,
    }
    impl IABDonorCUUPPSKInfoItem {
        pub fn new(
            i_ab_donor_cu_uppsk: IABDonorCUUPPSK,
            i_ab_donor_cu_upipaddress: TransportLayerAddress,
            i_ab_duipaddress: TransportLayerAddress,
            i_e_extensions: Option<IABDonorCUUPPSKInfoItemIEExtensions>,
        ) -> Self {
            Self {
                i_ab_donor_cu_uppsk,
                i_ab_donor_cu_upipaddress,
                i_ab_duipaddress,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "IAB-donor-CU-UPPSK")]
    pub struct IABDonorCUUPPSK(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum IgnoreMappingRuleIndication {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousImmediateMDTIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousImmediateMDTIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousImmediateMDTIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousImmediateMDTIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousImmediateMDTIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct ImmediateMDTIEExtensions(pub SequenceOf<AnonymousImmediateMDTIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ImmediateMDT {
        #[rasn(identifier = "measurementsToActivate")]
        pub measurements_to_activate: MeasurementsToActivate,
        #[rasn(identifier = "measurementFour")]
        pub measurement_four: Option<M4Configuration>,
        #[rasn(identifier = "measurementSix")]
        pub measurement_six: Option<M6Configuration>,
        #[rasn(identifier = "measurementSeven")]
        pub measurement_seven: Option<M7Configuration>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ImmediateMDTIEExtensions>,
    }
    impl ImmediateMDT {
        pub fn new(
            measurements_to_activate: MeasurementsToActivate,
            measurement_four: Option<M4Configuration>,
            measurement_six: Option<M6Configuration>,
            measurement_seven: Option<M7Configuration>,
            i_e_extensions: Option<ImmediateMDTIEExtensions>,
        ) -> Self {
            Self {
                measurements_to_activate,
                measurement_four,
                measurement_six,
                measurement_seven,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "Inactivity-Timer",
        value("1..=7200", extensible)
    )]
    pub struct InactivityTimer(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum InactivityInformationRequest {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[doc = " I "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum IndirectPathIndication {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum IntegrityProtectionAlgorithm {
        nIA0 = 0,
        #[rasn(identifier = "i-128-NIA1")]
        i_128_NIA1 = 1,
        #[rasn(identifier = "i-128-NIA2")]
        i_128_NIA2 = 2,
        #[rasn(identifier = "i-128-NIA3")]
        i_128_NIA3 = 3,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum IntegrityProtectionIndication {
        required = 0,
        preferred = 1,
        #[rasn(identifier = "not-needed")]
        not_needed = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct IntegrityProtectionKey(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum IntegrityProtectionResult {
        performed = 0,
        #[rasn(identifier = "not-performed")]
        not_performed = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct InterfacesToTrace(pub FixedBitString<8usize>);
    #[doc = " J"]
    #[doc = " K"]
    #[doc = " L"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "Links-to-log")]
    #[non_exhaustive]
    pub enum LinksToLog {
        uplink = 0,
        downlink = 1,
        #[rasn(identifier = "both-uplink-and-downlink")]
        both_uplink_and_downlink = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"))]
    pub struct LocationDependentMBSF1UInformationAtCU(
        pub SequenceOf<LocationDependentMBSF1UInformationAtCUItem>,
    );
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousLocationDependentMBSF1UInformationAtCUItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousLocationDependentMBSF1UInformationAtCUItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousLocationDependentMBSF1UInformationAtCUItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousLocationDependentMBSF1UInformationAtCUItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousLocationDependentMBSF1UInformationAtCUItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct LocationDependentMBSF1UInformationAtCUItemIEExtensions(
        pub SequenceOf<AnonymousLocationDependentMBSF1UInformationAtCUItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "LocationDependentMBSF1UInformationAtCU-Item"
    )]
    #[non_exhaustive]
    pub struct LocationDependentMBSF1UInformationAtCUItem {
        #[rasn(identifier = "mbsAreaSession-ID")]
        pub mbs_area_session_id: MBSAreaSessionID,
        #[rasn(identifier = "mbs-f1u-info-at-CU")]
        pub mbs_f1u_info_at_cu: UPTNLInformation,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<LocationDependentMBSF1UInformationAtCUItemIEExtensions>,
    }
    impl LocationDependentMBSF1UInformationAtCUItem {
        pub fn new(
            mbs_area_session_id: MBSAreaSessionID,
            mbs_f1u_info_at_cu: UPTNLInformation,
            i_e_extensions: Option<LocationDependentMBSF1UInformationAtCUItemIEExtensions>,
        ) -> Self {
            Self {
                mbs_area_session_id,
                mbs_f1u_info_at_cu,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"))]
    pub struct LocationDependentMBSF1UInformationAtDU(
        pub SequenceOf<LocationDependentMBSF1UInformationAtDUItem>,
    );
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousLocationDependentMBSF1UInformationAtDUItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousLocationDependentMBSF1UInformationAtDUItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousLocationDependentMBSF1UInformationAtDUItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousLocationDependentMBSF1UInformationAtDUItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousLocationDependentMBSF1UInformationAtDUItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct LocationDependentMBSF1UInformationAtDUItemIEExtensions(
        pub SequenceOf<AnonymousLocationDependentMBSF1UInformationAtDUItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "LocationDependentMBSF1UInformationAtDU-Item"
    )]
    #[non_exhaustive]
    pub struct LocationDependentMBSF1UInformationAtDUItem {
        #[rasn(identifier = "mbsAreaSession-ID")]
        pub mbs_area_session_id: MBSAreaSessionID,
        #[rasn(identifier = "mbs-f1u-info-at-DU")]
        pub mbs_f1u_info_at_du: UPTNLInformation,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<LocationDependentMBSF1UInformationAtDUItemIEExtensions>,
    }
    impl LocationDependentMBSF1UInformationAtDUItem {
        pub fn new(
            mbs_area_session_id: MBSAreaSessionID,
            mbs_f1u_info_at_du: UPTNLInformation,
            i_e_extensions: Option<LocationDependentMBSF1UInformationAtDUItemIEExtensions>,
        ) -> Self {
            Self {
                mbs_area_session_id,
                mbs_f1u_info_at_du,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"))]
    pub struct LocationDependentMBSNGUInformationAt5GC(
        pub SequenceOf<LocationDependentMBSNGUInformationAt5GCItem>,
    );
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousLocationDependentMBSNGUInformationAt5GCItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousLocationDependentMBSNGUInformationAt5GCItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality:
            AnonymousLocationDependentMBSNGUInformationAt5GCItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousLocationDependentMBSNGUInformationAt5GCItemIEExtensions {
        pub fn new(
            id: u16,
            criticality : AnonymousLocationDependentMBSNGUInformationAt5GCItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct LocationDependentMBSNGUInformationAt5GCItemIEExtensions(
        pub SequenceOf<AnonymousLocationDependentMBSNGUInformationAt5GCItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "LocationDependentMBSNGUInformationAt5GC-Item"
    )]
    #[non_exhaustive]
    pub struct LocationDependentMBSNGUInformationAt5GCItem {
        #[rasn(identifier = "mbsAreaSession-ID")]
        pub mbs_area_session_id: MBSAreaSessionID,
        #[rasn(identifier = "mbsNGUInformationAt5GC")]
        pub mbs_nguinformation_at5_gc: MBSNGUInformationAt5GC,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<LocationDependentMBSNGUInformationAt5GCItemIEExtensions>,
    }
    impl LocationDependentMBSNGUInformationAt5GCItem {
        pub fn new(
            mbs_area_session_id: MBSAreaSessionID,
            mbs_nguinformation_at5_gc: MBSNGUInformationAt5GC,
            i_e_extensions: Option<LocationDependentMBSNGUInformationAt5GCItemIEExtensions>,
        ) -> Self {
            Self {
                mbs_area_session_id,
                mbs_nguinformation_at5_gc,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"))]
    pub struct LocationDependentMBSNGUInformationAtNGRAN(
        pub SequenceOf<LocationDependentMBSNGUInformationAtNGRANItem>,
    );
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousLocationDependentMBSNGUInformationAtNGRANItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousLocationDependentMBSNGUInformationAtNGRANItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality:
            AnonymousLocationDependentMBSNGUInformationAtNGRANItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousLocationDependentMBSNGUInformationAtNGRANItemIEExtensions {
        pub fn new(
            id: u16,
            criticality : AnonymousLocationDependentMBSNGUInformationAtNGRANItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct LocationDependentMBSNGUInformationAtNGRANItemIEExtensions(
        pub SequenceOf<AnonymousLocationDependentMBSNGUInformationAtNGRANItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "LocationDependentMBSNGUInformationAtNGRAN-Item"
    )]
    #[non_exhaustive]
    pub struct LocationDependentMBSNGUInformationAtNGRANItem {
        #[rasn(identifier = "mbsAreaSession-ID")]
        pub mbs_area_session_id: MBSAreaSessionID,
        #[rasn(identifier = "mbsNGUInformationAtNGRAN")]
        pub mbs_nguinformation_at_ngran: MBSNGUInformationAtNGRAN,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<LocationDependentMBSNGUInformationAtNGRANItemIEExtensions>,
    }
    impl LocationDependentMBSNGUInformationAtNGRANItem {
        pub fn new(
            mbs_area_session_id: MBSAreaSessionID,
            mbs_nguinformation_at_ngran: MBSNGUInformationAtNGRAN,
            i_e_extensions: Option<LocationDependentMBSNGUInformationAtNGRANItemIEExtensions>,
        ) -> Self {
            Self {
                mbs_area_session_id,
                mbs_nguinformation_at_ngran,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousM4ConfigurationIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousM4ConfigurationIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousM4ConfigurationIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousM4ConfigurationIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousM4ConfigurationIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct M4ConfigurationIEExtensions(pub SequenceOf<AnonymousM4ConfigurationIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct M4Configuration {
        pub m4period: M4period,
        #[rasn(identifier = "m4-links-to-log")]
        pub m4_links_to_log: LinksToLog,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<M4ConfigurationIEExtensions>,
    }
    impl M4Configuration {
        pub fn new(
            m4period: M4period,
            m4_links_to_log: LinksToLog,
            i_e_extensions: Option<M4ConfigurationIEExtensions>,
        ) -> Self {
            Self {
                m4period,
                m4_links_to_log,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum M4ReportAmount {
        r1 = 0,
        r2 = 1,
        r4 = 2,
        r8 = 3,
        r16 = 4,
        r32 = 5,
        r64 = 6,
        infinity = 7,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum M4period {
        ms1024 = 0,
        ms2048 = 1,
        ms5120 = 2,
        ms10240 = 3,
        min1 = 4,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousM6ConfigurationIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousM6ConfigurationIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousM6ConfigurationIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousM6ConfigurationIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousM6ConfigurationIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct M6ConfigurationIEExtensions(pub SequenceOf<AnonymousM6ConfigurationIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct M6Configuration {
        #[rasn(identifier = "m6report-Interval")]
        pub m6report_interval: M6reportInterval,
        #[rasn(identifier = "m6-links-to-log")]
        pub m6_links_to_log: LinksToLog,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<M6ConfigurationIEExtensions>,
    }
    impl M6Configuration {
        pub fn new(
            m6report_interval: M6reportInterval,
            m6_links_to_log: LinksToLog,
            i_e_extensions: Option<M6ConfigurationIEExtensions>,
        ) -> Self {
            Self {
                m6report_interval,
                m6_links_to_log,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum M6ReportAmount {
        r1 = 0,
        r2 = 1,
        r4 = 2,
        r8 = 3,
        r16 = 4,
        r32 = 5,
        r64 = 6,
        infinity = 7,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "M6report-Interval")]
    #[non_exhaustive]
    pub enum M6reportInterval {
        ms120 = 0,
        ms240 = 1,
        ms480 = 2,
        ms640 = 3,
        ms1024 = 4,
        ms2048 = 5,
        ms5120 = 6,
        ms10240 = 7,
        ms20480 = 8,
        ms40960 = 9,
        min1 = 10,
        min6 = 11,
        min12 = 12,
        min30 = 13,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousM7ConfigurationIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousM7ConfigurationIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousM7ConfigurationIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousM7ConfigurationIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousM7ConfigurationIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct M7ConfigurationIEExtensions(pub SequenceOf<AnonymousM7ConfigurationIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct M7Configuration {
        pub m7period: M7period,
        #[rasn(identifier = "m7-links-to-log")]
        pub m7_links_to_log: LinksToLog,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<M7ConfigurationIEExtensions>,
    }
    impl M7Configuration {
        pub fn new(
            m7period: M7period,
            m7_links_to_log: LinksToLog,
            i_e_extensions: Option<M7ConfigurationIEExtensions>,
        ) -> Self {
            Self {
                m7period,
                m7_links_to_log,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum M7ReportAmount {
        r1 = 0,
        r2 = 1,
        r4 = 2,
        r8 = 3,
        r16 = 4,
        r32 = 5,
        r64 = 6,
        infinity = 7,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=60", extensible))]
    pub struct M7period(pub Integer);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum MBSDLDataArrivalDlDataArrival {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMBSDLDataArrivalIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMBSDLDataArrivalIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMBSDLDataArrivalIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMBSDLDataArrivalIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMBSDLDataArrivalIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MBSDLDataArrivalIEExtensions(pub SequenceOf<AnonymousMBSDLDataArrivalIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MBS-DL-Data-Arrival")]
    #[non_exhaustive]
    pub struct MBSDLDataArrival {
        #[rasn(identifier = "dlDataArrival")]
        pub dl_data_arrival: MBSDLDataArrivalDlDataArrival,
        pub ppi: Option<PPI>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MBSDLDataArrivalIEExtensions>,
    }
    impl MBSDLDataArrival {
        pub fn new(
            dl_data_arrival: MBSDLDataArrivalDlDataArrival,
            ppi: Option<PPI>,
            i_e_extensions: Option<MBSDLDataArrivalIEExtensions>,
        ) -> Self {
            Self {
                dl_data_arrival,
                ppi,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "MBS-PDCP-COUNT")]
    pub struct MBSPDCPCOUNT(pub FixedBitString<32usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "MBS-PDCP-COUNT-Req")]
    #[non_exhaustive]
    pub enum MBSPDCPCOUNTReq {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMBSServiceAreaIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMBSServiceAreaIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMBSServiceAreaIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMBSServiceAreaIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMBSServiceAreaIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MBSServiceAreaIEExtensions(pub SequenceOf<AnonymousMBSServiceAreaIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MBS-ServiceArea")]
    #[non_exhaustive]
    pub struct MBSServiceArea {
        #[rasn(identifier = "mBS-ServiceAreaInformationList")]
        pub m_bs_service_area_information_list: Option<MBSServiceAreaInformationList>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MBSServiceAreaIEExtensions>,
    }
    impl MBSServiceArea {
        pub fn new(
            m_bs_service_area_information_list: Option<MBSServiceAreaInformationList>,
            i_e_extensions: Option<MBSServiceAreaIEExtensions>,
        ) -> Self {
            Self {
                m_bs_service_area_information_list,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=512"), identifier = "MBS-ServiceAreaCellList")]
    pub struct MBSServiceAreaCellList(pub SequenceOf<NRCGI>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMBSServiceAreaInformationIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMBSServiceAreaInformationIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMBSServiceAreaInformationIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMBSServiceAreaInformationIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMBSServiceAreaInformationIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MBSServiceAreaInformationIEExtensions(
        pub SequenceOf<AnonymousMBSServiceAreaInformationIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MBS-ServiceAreaInformation")]
    #[non_exhaustive]
    pub struct MBSServiceAreaInformation {
        #[rasn(identifier = "mBS-ServiceAreaCellList")]
        pub m_bs_service_area_cell_list: Option<MBSServiceAreaCellList>,
        #[rasn(identifier = "mBS-ServiceAreaTAIList")]
        pub m_bs_service_area_tailist: Option<MBSServiceAreaTAIList>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MBSServiceAreaInformationIEExtensions>,
    }
    impl MBSServiceAreaInformation {
        pub fn new(
            m_bs_service_area_cell_list: Option<MBSServiceAreaCellList>,
            m_bs_service_area_tailist: Option<MBSServiceAreaTAIList>,
            i_e_extensions: Option<MBSServiceAreaInformationIEExtensions>,
        ) -> Self {
            Self {
                m_bs_service_area_cell_list,
                m_bs_service_area_tailist,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMBSServiceAreaInformationItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMBSServiceAreaInformationItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMBSServiceAreaInformationItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMBSServiceAreaInformationItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMBSServiceAreaInformationItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MBSServiceAreaInformationItemIEExtensions(
        pub SequenceOf<AnonymousMBSServiceAreaInformationItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MBS-ServiceAreaInformationItem")]
    #[non_exhaustive]
    pub struct MBSServiceAreaInformationItem {
        #[rasn(identifier = "mBS-AreaSessionID")]
        pub m_bs_area_session_id: MBSAreaSessionID,
        #[rasn(identifier = "mBS-ServiceAreaInformation")]
        pub m_bs_service_area_information: MBSServiceAreaInformation,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MBSServiceAreaInformationItemIEExtensions>,
    }
    impl MBSServiceAreaInformationItem {
        pub fn new(
            m_bs_area_session_id: MBSAreaSessionID,
            m_bs_service_area_information: MBSServiceAreaInformation,
            i_e_extensions: Option<MBSServiceAreaInformationItemIEExtensions>,
        ) -> Self {
            Self {
                m_bs_area_session_id,
                m_bs_service_area_information,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "MBS-ServiceAreaInformationList"
    )]
    pub struct MBSServiceAreaInformationList(pub SequenceOf<MBSServiceAreaInformationItem>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=512"), identifier = "MBS-ServiceAreaTAIList")]
    pub struct MBSServiceAreaTAIList(pub SequenceOf<MBSServiceAreaTAIListItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMBSServiceAreaTAIListItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMBSServiceAreaTAIListItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMBSServiceAreaTAIListItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMBSServiceAreaTAIListItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMBSServiceAreaTAIListItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MBSServiceAreaTAIListItemIEExtensions(
        pub SequenceOf<AnonymousMBSServiceAreaTAIListItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MBS-ServiceAreaTAIList-Item")]
    #[non_exhaustive]
    pub struct MBSServiceAreaTAIListItem {
        #[rasn(identifier = "plmn-ID")]
        pub plmn_id: PLMNIdentity,
        #[rasn(identifier = "fiveGS-TAC")]
        pub five_gs_tac: FiveGSTAC,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MBSServiceAreaTAIListItemIEExtensions>,
    }
    impl MBSServiceAreaTAIListItem {
        pub fn new(
            plmn_id: PLMNIdentity,
            five_gs_tac: FiveGSTAC,
            i_e_extensions: Option<MBSServiceAreaTAIListItemIEExtensions>,
        ) -> Self {
            Self {
                plmn_id,
                five_gs_tac,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMBSSupportInfoToAddItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMBSSupportInfoToAddItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMBSSupportInfoToAddItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMBSSupportInfoToAddItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMBSSupportInfoToAddItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MBSSupportInfoToAddItemIEExtensions(
        pub SequenceOf<AnonymousMBSSupportInfoToAddItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MBS-Support-Info-ToAdd-Item")]
    #[non_exhaustive]
    pub struct MBSSupportInfoToAddItem {
        #[rasn(identifier = "globalMBSSessionID")]
        pub global_mbssession_id: GlobalMBSSessionID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MBSSupportInfoToAddItemIEExtensions>,
    }
    impl MBSSupportInfoToAddItem {
        pub fn new(
            global_mbssession_id: GlobalMBSSessionID,
            i_e_extensions: Option<MBSSupportInfoToAddItemIEExtensions>,
        ) -> Self {
            Self {
                global_mbssession_id,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=512"), identifier = "MBS-Support-Info-ToAdd-List")]
    pub struct MBSSupportInfoToAddList(pub SequenceOf<MBSSupportInfoToAddItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMBSSupportInfoToRemoveItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMBSSupportInfoToRemoveItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMBSSupportInfoToRemoveItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMBSSupportInfoToRemoveItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMBSSupportInfoToRemoveItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MBSSupportInfoToRemoveItemIEExtensions(
        pub SequenceOf<AnonymousMBSSupportInfoToRemoveItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MBS-Support-Info-ToRemove-Item")]
    #[non_exhaustive]
    pub struct MBSSupportInfoToRemoveItem {
        #[rasn(identifier = "globalMBSSessionID")]
        pub global_mbssession_id: GlobalMBSSessionID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MBSSupportInfoToRemoveItemIEExtensions>,
    }
    impl MBSSupportInfoToRemoveItem {
        pub fn new(
            global_mbssession_id: GlobalMBSSessionID,
            i_e_extensions: Option<MBSSupportInfoToRemoveItemIEExtensions>,
        ) -> Self {
            Self {
                global_mbssession_id,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=512"),
        identifier = "MBS-Support-Info-ToRemove-List"
    )]
    pub struct MBSSupportInfoToRemoveList(pub SequenceOf<MBSSupportInfoToRemoveItem>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535", extensible))]
    pub struct MBSAreaSessionID(pub Integer);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMBSF1UInformationAtCUIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMBSF1UInformationAtCUIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMBSF1UInformationAtCUIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMBSF1UInformationAtCUIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMBSF1UInformationAtCUIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MBSF1UInformationAtCUIEExtensions(
        pub SequenceOf<AnonymousMBSF1UInformationAtCUIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MBSF1UInformationAtCU {
        #[rasn(identifier = "mbs-f1u-info-at-CU")]
        pub mbs_f1u_info_at_cu: UPTNLInformation,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MBSF1UInformationAtCUIEExtensions>,
    }
    impl MBSF1UInformationAtCU {
        pub fn new(
            mbs_f1u_info_at_cu: UPTNLInformation,
            i_e_extensions: Option<MBSF1UInformationAtCUIEExtensions>,
        ) -> Self {
            Self {
                mbs_f1u_info_at_cu,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMBSF1UInformationAtDUIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMBSF1UInformationAtDUIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMBSF1UInformationAtDUIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMBSF1UInformationAtDUIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMBSF1UInformationAtDUIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MBSF1UInformationAtDUIEExtensions(
        pub SequenceOf<AnonymousMBSF1UInformationAtDUIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MBSF1UInformationAtDU {
        #[rasn(identifier = "mbs-f1u-info-at-DU")]
        pub mbs_f1u_info_at_du: UPTNLInformation,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MBSF1UInformationAtDUIEExtensions>,
    }
    impl MBSF1UInformationAtDU {
        pub fn new(
            mbs_f1u_info_at_du: UPTNLInformation,
            i_e_extensions: Option<MBSF1UInformationAtDUIEExtensions>,
        ) -> Self {
            Self {
                mbs_f1u_info_at_du,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum MBSMulticastF1UContextDescriptorMcF1UCtxtusage {
        ptm = 0,
        ptp = 1,
        #[rasn(identifier = "ptp-retransmission")]
        ptp_retransmission = 2,
        #[rasn(identifier = "ptp-forwarding")]
        ptp_forwarding = 3,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMBSMulticastF1UContextDescriptorIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMBSMulticastF1UContextDescriptorIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMBSMulticastF1UContextDescriptorIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMBSMulticastF1UContextDescriptorIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMBSMulticastF1UContextDescriptorIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MBSMulticastF1UContextDescriptorIEExtensions(
        pub SequenceOf<AnonymousMBSMulticastF1UContextDescriptorIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MBSMulticastF1UContextDescriptor {
        #[rasn(identifier = "multicastF1UContextReferenceE1")]
        pub multicast_f1_ucontext_reference_e1: MulticastF1UContextReferenceE1,
        #[rasn(identifier = "mc-F1UCtxtusage")]
        pub mc_f1_uctxtusage: MBSMulticastF1UContextDescriptorMcF1UCtxtusage,
        #[rasn(identifier = "mbsAreaSession")]
        pub mbs_area_session: Option<MBSAreaSessionID>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MBSMulticastF1UContextDescriptorIEExtensions>,
    }
    impl MBSMulticastF1UContextDescriptor {
        pub fn new(
            multicast_f1_ucontext_reference_e1: MulticastF1UContextReferenceE1,
            mc_f1_uctxtusage: MBSMulticastF1UContextDescriptorMcF1UCtxtusage,
            mbs_area_session: Option<MBSAreaSessionID>,
            i_e_extensions: Option<MBSMulticastF1UContextDescriptorIEExtensions>,
        ) -> Self {
            Self {
                multicast_f1_ucontext_reference_e1,
                mc_f1_uctxtusage,
                mbs_area_session,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum MBSNGUInformationAt5GCChoiceExtensionCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct MBSNGUInformationAt5GCChoiceExtension {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: MBSNGUInformationAt5GCChoiceExtensionCriticality,
        pub value: Any,
    }
    impl MBSNGUInformationAt5GCChoiceExtension {
        pub fn new(
            id: u16,
            criticality: MBSNGUInformationAt5GCChoiceExtensionCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum MBSNGUInformationAt5GC {
        multicast(MBSNGUInformationAt5GCMulticast),
        #[rasn(identifier = "choice-extension")]
        choice_extension(MBSNGUInformationAt5GCChoiceExtension),
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMBSNGUInformationAt5GCMulticastIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMBSNGUInformationAt5GCMulticastIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMBSNGUInformationAt5GCMulticastIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMBSNGUInformationAt5GCMulticastIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMBSNGUInformationAt5GCMulticastIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MBSNGUInformationAt5GCMulticastIEExtensions(
        pub SequenceOf<AnonymousMBSNGUInformationAt5GCMulticastIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MBSNGUInformationAt5GC-Multicast")]
    #[non_exhaustive]
    pub struct MBSNGUInformationAt5GCMulticast {
        #[rasn(identifier = "ipmcAddress")]
        pub ipmc_address: TransportLayerAddress,
        #[rasn(identifier = "ipsourceAddress")]
        pub ipsource_address: TransportLayerAddress,
        #[rasn(identifier = "gtpDLTEID")]
        pub gtp_dlteid: GTPTEID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MBSNGUInformationAt5GCMulticastIEExtensions>,
    }
    impl MBSNGUInformationAt5GCMulticast {
        pub fn new(
            ipmc_address: TransportLayerAddress,
            ipsource_address: TransportLayerAddress,
            gtp_dlteid: GTPTEID,
            i_e_extensions: Option<MBSNGUInformationAt5GCMulticastIEExtensions>,
        ) -> Self {
            Self {
                ipmc_address,
                ipsource_address,
                gtp_dlteid,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum MBSNGUInformationAtNGRANChoiceExtensionCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct MBSNGUInformationAtNGRANChoiceExtension {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: MBSNGUInformationAtNGRANChoiceExtensionCriticality,
        pub value: Any,
    }
    impl MBSNGUInformationAtNGRANChoiceExtension {
        pub fn new(
            id: u16,
            criticality: MBSNGUInformationAtNGRANChoiceExtensionCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum MBSNGUInformationAtNGRAN {
        unicast(UPTNLInformation),
        #[rasn(identifier = "choice-extension")]
        choice_extension(MBSNGUInformationAtNGRANChoiceExtension),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "MBSNGUInformationAtNGRAN-Request")]
    #[non_exhaustive]
    pub enum MBSNGUInformationAtNGRANRequest {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMBSNGUInformationAtNGRANRequestItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMBSNGUInformationAtNGRANRequestItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMBSNGUInformationAtNGRANRequestItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMBSNGUInformationAtNGRANRequestItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMBSNGUInformationAtNGRANRequestItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MBSNGUInformationAtNGRANRequestItemIEExtensions(
        pub SequenceOf<AnonymousMBSNGUInformationAtNGRANRequestItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MBSNGUInformationAtNGRAN-Request-Item")]
    #[non_exhaustive]
    pub struct MBSNGUInformationAtNGRANRequestItem {
        #[rasn(identifier = "mbsAreaSession-ID")]
        pub mbs_area_session_id: MBSAreaSessionID,
        #[rasn(identifier = "mbsNGUInformationAtNGRAN-Request")]
        pub mbs_nguinformation_at_ngran_request: MBSNGUInformationAtNGRANRequest,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MBSNGUInformationAtNGRANRequestItemIEExtensions>,
    }
    impl MBSNGUInformationAtNGRANRequestItem {
        pub fn new(
            mbs_area_session_id: MBSAreaSessionID,
            mbs_nguinformation_at_ngran_request: MBSNGUInformationAtNGRANRequest,
            i_e_extensions: Option<MBSNGUInformationAtNGRANRequestItemIEExtensions>,
        ) -> Self {
            Self {
                mbs_area_session_id,
                mbs_nguinformation_at_ngran_request,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "MBSNGUInformationAtNGRAN-Request-List"
    )]
    pub struct MBSNGUInformationAtNGRANRequestList(
        pub SequenceOf<MBSNGUInformationAtNGRANRequestItem>,
    );
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMBSSessionAssociatedInfoNonSupportToSupportIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMBSSessionAssociatedInfoNonSupportToSupportIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality:
            AnonymousMBSSessionAssociatedInfoNonSupportToSupportIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMBSSessionAssociatedInfoNonSupportToSupportIEExtensions {
        pub fn new(
            id: u16,
            criticality : AnonymousMBSSessionAssociatedInfoNonSupportToSupportIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MBSSessionAssociatedInfoNonSupportToSupportIEExtensions(
        pub SequenceOf<AnonymousMBSSessionAssociatedInfoNonSupportToSupportIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MBSSessionAssociatedInfoNonSupportToSupport {
        #[rasn(identifier = "ue-Reference-ID")]
        pub ue_reference_id: GNBCUCPUEE1APID,
        #[rasn(identifier = "pDU-Session-ID")]
        pub p_du_session_id: PDUSessionID,
        #[rasn(identifier = "associatedQoSFlowInformationList")]
        pub associated_qo_sflow_information_list: MBSSessionAssociatedInformationList,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MBSSessionAssociatedInfoNonSupportToSupportIEExtensions>,
    }
    impl MBSSessionAssociatedInfoNonSupportToSupport {
        pub fn new(
            ue_reference_id: GNBCUCPUEE1APID,
            p_du_session_id: PDUSessionID,
            associated_qo_sflow_information_list: MBSSessionAssociatedInformationList,
            i_e_extensions: Option<MBSSessionAssociatedInfoNonSupportToSupportIEExtensions>,
        ) -> Self {
            Self {
                ue_reference_id,
                p_du_session_id,
                associated_qo_sflow_information_list,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMBSSessionAssociatedInformationIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMBSSessionAssociatedInformationIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMBSSessionAssociatedInformationIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMBSSessionAssociatedInformationIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMBSSessionAssociatedInformationIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MBSSessionAssociatedInformationIEExtensions(
        pub SequenceOf<AnonymousMBSSessionAssociatedInformationIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MBSSessionAssociatedInformation {
        #[rasn(identifier = "mbsSessionAssociatedInformationList")]
        pub mbs_session_associated_information_list: MBSSessionAssociatedInformationList,
        #[rasn(identifier = "mbsSessionForwardingAddress")]
        pub mbs_session_forwarding_address: UPTNLInformation,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MBSSessionAssociatedInformationIEExtensions>,
    }
    impl MBSSessionAssociatedInformation {
        pub fn new(
            mbs_session_associated_information_list: MBSSessionAssociatedInformationList,
            mbs_session_forwarding_address: UPTNLInformation,
            i_e_extensions: Option<MBSSessionAssociatedInformationIEExtensions>,
        ) -> Self {
            Self {
                mbs_session_associated_information_list,
                mbs_session_forwarding_address,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMBSSessionAssociatedInformationItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMBSSessionAssociatedInformationItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMBSSessionAssociatedInformationItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMBSSessionAssociatedInformationItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMBSSessionAssociatedInformationItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MBSSessionAssociatedInformationItemIEExtensions(
        pub SequenceOf<AnonymousMBSSessionAssociatedInformationItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MBSSessionAssociatedInformation-Item")]
    #[non_exhaustive]
    pub struct MBSSessionAssociatedInformationItem {
        #[rasn(identifier = "mbs-QoS-Flow-Identifier")]
        pub mbs_qo_s_flow_identifier: QoSFlowIdentifier,
        #[rasn(identifier = "associated-unicast-QoS-Flow-Identifier")]
        pub associated_unicast_qo_s_flow_identifier: QoSFlowIdentifier,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MBSSessionAssociatedInformationItemIEExtensions>,
    }
    impl MBSSessionAssociatedInformationItem {
        pub fn new(
            mbs_qo_s_flow_identifier: QoSFlowIdentifier,
            associated_unicast_qo_s_flow_identifier: QoSFlowIdentifier,
            i_e_extensions: Option<MBSSessionAssociatedInformationItemIEExtensions>,
        ) -> Self {
            Self {
                mbs_qo_s_flow_identifier,
                associated_unicast_qo_s_flow_identifier,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=64"))]
    pub struct MBSSessionAssociatedInformationList(
        pub SequenceOf<MBSSessionAssociatedInformationItem>,
    );
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum MBSSessionResourceNotificationChoiceExtensionCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct MBSSessionResourceNotificationChoiceExtension {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: MBSSessionResourceNotificationChoiceExtensionCriticality,
        pub value: Any,
    }
    impl MBSSessionResourceNotificationChoiceExtension {
        pub fn new(
            id: u16,
            criticality: MBSSessionResourceNotificationChoiceExtensionCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum MBSSessionResourceNotification {
        #[rasn(identifier = "mbs-DL-Data-Arrival")]
        mbs_DL_Data_Arrival(MBSDLDataArrival),
        inactivity(MCBearerContextInactivity),
        #[rasn(identifier = "choice-extension")]
        choice_extension(MBSSessionResourceNotificationChoiceExtension),
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum MCBearerContextInactivityMcBearerContextInactivityIndication {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCBearerContextInactivityIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCBearerContextInactivityIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCBearerContextInactivityIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMCBearerContextInactivityIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMCBearerContextInactivityIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MCBearerContextInactivityIEExtensions(
        pub SequenceOf<AnonymousMCBearerContextInactivityIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MCBearerContext-Inactivity")]
    #[non_exhaustive]
    pub struct MCBearerContextInactivity {
        #[rasn(identifier = "mcBearerContext-Inactivity-Indication")]
        pub mc_bearer_context_inactivity_indication:
            MCBearerContextInactivityMcBearerContextInactivityIndication,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MCBearerContextInactivityIEExtensions>,
    }
    impl MCBearerContextInactivity {
        pub fn new(
            mc_bearer_context_inactivity_indication : MCBearerContextInactivityMcBearerContextInactivityIndication,
            i_e_extensions: Option<MCBearerContextInactivityIEExtensions>,
        ) -> Self {
            Self {
                mc_bearer_context_inactivity_indication,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCBearerContextF1UTNLInfoatDUIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCBearerContextF1UTNLInfoatDUIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCBearerContextF1UTNLInfoatDUIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMCBearerContextF1UTNLInfoatDUIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMCBearerContextF1UTNLInfoatDUIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MCBearerContextF1UTNLInfoatDUIEExtensions(
        pub SequenceOf<AnonymousMCBearerContextF1UTNLInfoatDUIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCBearerContextF1UTNLInfoatDU {
        #[rasn(identifier = "mbsF1UInfoatDU")]
        pub mbs_f1_uinfoat_du: UPTNLInformation,
        #[rasn(identifier = "mbsMulticastF1UContextDescriptor")]
        pub mbs_multicast_f1_ucontext_descriptor: MBSMulticastF1UContextDescriptor,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MCBearerContextF1UTNLInfoatDUIEExtensions>,
    }
    impl MCBearerContextF1UTNLInfoatDU {
        pub fn new(
            mbs_f1_uinfoat_du: UPTNLInformation,
            mbs_multicast_f1_ucontext_descriptor: MBSMulticastF1UContextDescriptor,
            i_e_extensions: Option<MCBearerContextF1UTNLInfoatDUIEExtensions>,
        ) -> Self {
            Self {
                mbs_f1_uinfoat_du,
                mbs_multicast_f1_ucontext_descriptor,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum MCBearerContextNGUTNLInfoatNGRANChoiceExtensionCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct MCBearerContextNGUTNLInfoatNGRANChoiceExtension {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: MCBearerContextNGUTNLInfoatNGRANChoiceExtensionCriticality,
        pub value: Any,
    }
    impl MCBearerContextNGUTNLInfoatNGRANChoiceExtension {
        pub fn new(
            id: u16,
            criticality: MCBearerContextNGUTNLInfoatNGRANChoiceExtensionCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        choice,
        automatic_tags,
        identifier = "MCBearerContextNGU-TNLInfoatNGRAN"
    )]
    pub enum MCBearerContextNGUTNLInfoatNGRAN {
        locationindependent(MBSNGUInformationAtNGRAN),
        locationdependent(LocationDependentMBSNGUInformationAtNGRAN),
        #[rasn(identifier = "choice-extension")]
        choice_extension(MCBearerContextNGUTNLInfoatNGRANChoiceExtension),
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCBearerContextNGUTNLInfoatNGRANModifyResponseIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCBearerContextNGUTNLInfoatNGRANModifyResponseIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality:
            AnonymousMCBearerContextNGUTNLInfoatNGRANModifyResponseIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMCBearerContextNGUTNLInfoatNGRANModifyResponseIEExtensions {
        pub fn new(
            id: u16,
            criticality : AnonymousMCBearerContextNGUTNLInfoatNGRANModifyResponseIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MCBearerContextNGUTNLInfoatNGRANModifyResponseIEExtensions(
        pub SequenceOf<AnonymousMCBearerContextNGUTNLInfoatNGRANModifyResponseIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "MCBearerContextNGU-TNLInfoatNGRANModifyResponse"
    )]
    #[non_exhaustive]
    pub struct MCBearerContextNGUTNLInfoatNGRANModifyResponse {
        #[rasn(identifier = "mbs-NGU-InfoatNGRAN")]
        pub mbs_ngu_infoat_ngran: MBSNGUInformationAtNGRAN,
        #[rasn(identifier = "mbsAreaSession")]
        pub mbs_area_session: Option<MBSAreaSessionID>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MCBearerContextNGUTNLInfoatNGRANModifyResponseIEExtensions>,
    }
    impl MCBearerContextNGUTNLInfoatNGRANModifyResponse {
        pub fn new(
            mbs_ngu_infoat_ngran: MBSNGUInformationAtNGRAN,
            mbs_area_session: Option<MBSAreaSessionID>,
            i_e_extensions: Option<MCBearerContextNGUTNLInfoatNGRANModifyResponseIEExtensions>,
        ) -> Self {
            Self {
                mbs_ngu_infoat_ngran,
                mbs_area_session,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCBearerContextNGUTNLInfoat5GCIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCBearerContextNGUTNLInfoat5GCIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCBearerContextNGUTNLInfoat5GCIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMCBearerContextNGUTNLInfoat5GCIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMCBearerContextNGUTNLInfoat5GCIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MCBearerContextNGUTNLInfoat5GCIEExtensions(
        pub SequenceOf<AnonymousMCBearerContextNGUTNLInfoat5GCIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCBearerContextNGUTNLInfoat5GC {
        #[rasn(identifier = "mbsNGUInformationAt5GC")]
        pub mbs_nguinformation_at5_gc: MBSNGUInformationAt5GC,
        #[rasn(identifier = "mbsAreaSession-ID")]
        pub mbs_area_session_id: Option<MBSAreaSessionID>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MCBearerContextNGUTNLInfoat5GCIEExtensions>,
    }
    impl MCBearerContextNGUTNLInfoat5GC {
        pub fn new(
            mbs_nguinformation_at5_gc: MBSNGUInformationAt5GC,
            mbs_area_session_id: Option<MBSAreaSessionID>,
            i_e_extensions: Option<MCBearerContextNGUTNLInfoat5GCIEExtensions>,
        ) -> Self {
            Self {
                mbs_nguinformation_at5_gc,
                mbs_area_session_id,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum MCBearerContextNGUTnlInfoatNGRANRequestNgRANNGUTNLRequested {
        requested = 0,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCBearerContextNGUTnlInfoatNGRANRequestIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCBearerContextNGUTnlInfoatNGRANRequestIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCBearerContextNGUTnlInfoatNGRANRequestIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMCBearerContextNGUTnlInfoatNGRANRequestIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMCBearerContextNGUTnlInfoatNGRANRequestIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MCBearerContextNGUTnlInfoatNGRANRequestIEExtensions(
        pub SequenceOf<AnonymousMCBearerContextNGUTnlInfoatNGRANRequestIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCBearerContextNGUTnlInfoatNGRANRequest {
        #[rasn(identifier = "ngRANNGUTNLRequested")]
        pub ng_ranngutnlrequested: MCBearerContextNGUTnlInfoatNGRANRequestNgRANNGUTNLRequested,
        #[rasn(identifier = "mbsAreaSession-ID")]
        pub mbs_area_session_id: Option<MBSAreaSessionID>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MCBearerContextNGUTnlInfoatNGRANRequestIEExtensions>,
    }
    impl MCBearerContextNGUTnlInfoatNGRANRequest {
        pub fn new(
            ng_ranngutnlrequested: MCBearerContextNGUTnlInfoatNGRANRequestNgRANNGUTNLRequested,
            mbs_area_session_id: Option<MBSAreaSessionID>,
            i_e_extensions: Option<MCBearerContextNGUTnlInfoatNGRANRequestIEExtensions>,
        ) -> Self {
            Self {
                ng_ranngutnlrequested,
                mbs_area_session_id,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum MCBearerContextStatusChange {
        suspend = 0,
        resume = 1,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCBearerContextToModifyIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCBearerContextToModifyIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCBearerContextToModifyIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMCBearerContextToModifyIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMCBearerContextToModifyIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MCBearerContextToModifyIEExtensions(
        pub SequenceOf<AnonymousMCBearerContextToModifyIEExtensions>,
    );
    #[doc = " MCBearerContextToModify"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCBearerContextToModify {
        #[rasn(identifier = "mcBearerContextNGUTNLInfoat5GC")]
        pub mc_bearer_context_ngutnlinfoat5_gc: Option<MCBearerContextNGUTNLInfoat5GC>,
        #[rasn(identifier = "mcBearerContextNGUTnlInfoatNGRANRequest")]
        pub mc_bearer_context_ngutnl_infoat_ngranrequest:
            Option<MCBearerContextNGUTnlInfoatNGRANRequest>,
        #[rasn(identifier = "mbsMulticastF1UContextDescriptor")]
        pub mbs_multicast_f1_ucontext_descriptor: Option<MBSMulticastF1UContextDescriptor>,
        #[rasn(identifier = "requestedAction")]
        pub requested_action: Option<RequestedAction4AvailNGUTermination>,
        #[rasn(identifier = "mcMRBToSetupModifyList")]
        pub mc_mrbto_setup_modify_list: Option<MCMRBSetupModifyConfiguration>,
        #[rasn(identifier = "mcMRBToRemoveList")]
        pub mc_mrbto_remove_list: Option<MCMRBRemoveConfiguration>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MCBearerContextToModifyIEExtensions>,
    }
    impl MCBearerContextToModify {
        pub fn new(
            mc_bearer_context_ngutnlinfoat5_gc: Option<MCBearerContextNGUTNLInfoat5GC>,
            mc_bearer_context_ngutnl_infoat_ngranrequest: Option<
                MCBearerContextNGUTnlInfoatNGRANRequest,
            >,
            mbs_multicast_f1_ucontext_descriptor: Option<MBSMulticastF1UContextDescriptor>,
            requested_action: Option<RequestedAction4AvailNGUTermination>,
            mc_mrbto_setup_modify_list: Option<MCMRBSetupModifyConfiguration>,
            mc_mrbto_remove_list: Option<MCMRBRemoveConfiguration>,
            i_e_extensions: Option<MCBearerContextToModifyIEExtensions>,
        ) -> Self {
            Self {
                mc_bearer_context_ngutnlinfoat5_gc,
                mc_bearer_context_ngutnl_infoat_ngranrequest,
                mbs_multicast_f1_ucontext_descriptor,
                requested_action,
                mc_mrbto_setup_modify_list,
                mc_mrbto_remove_list,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCBearerContextToModifyConfirmIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCBearerContextToModifyConfirmIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCBearerContextToModifyConfirmIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMCBearerContextToModifyConfirmIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMCBearerContextToModifyConfirmIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MCBearerContextToModifyConfirmIEExtensions(
        pub SequenceOf<AnonymousMCBearerContextToModifyConfirmIEExtensions>,
    );
    #[doc = " MCBearerContextToModifyConfirm"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCBearerContextToModifyConfirm {
        #[rasn(identifier = "mbsMulticastF1UContextDescriptor")]
        pub mbs_multicast_f1_ucontext_descriptor: Option<MBSMulticastF1UContextDescriptor>,
        #[rasn(identifier = "mcMRBModifyConfirmList")]
        pub mc_mrbmodify_confirm_list: Option<MCMRBModifyConfirmList>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MCBearerContextToModifyConfirmIEExtensions>,
    }
    impl MCBearerContextToModifyConfirm {
        pub fn new(
            mbs_multicast_f1_ucontext_descriptor: Option<MBSMulticastF1UContextDescriptor>,
            mc_mrbmodify_confirm_list: Option<MCMRBModifyConfirmList>,
            i_e_extensions: Option<MCBearerContextToModifyConfirmIEExtensions>,
        ) -> Self {
            Self {
                mbs_multicast_f1_ucontext_descriptor,
                mc_mrbmodify_confirm_list,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCBearerContextToModifyRequiredIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCBearerContextToModifyRequiredIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCBearerContextToModifyRequiredIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMCBearerContextToModifyRequiredIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMCBearerContextToModifyRequiredIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MCBearerContextToModifyRequiredIEExtensions(
        pub SequenceOf<AnonymousMCBearerContextToModifyRequiredIEExtensions>,
    );
    #[doc = " MCBearerContextToModifyRequired"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCBearerContextToModifyRequired {
        #[rasn(identifier = "mbsMulticastF1UContextDescriptor")]
        pub mbs_multicast_f1_ucontext_descriptor: Option<MBSMulticastF1UContextDescriptor>,
        #[rasn(identifier = "mcMRBToRemoveRequiredList")]
        pub mc_mrbto_remove_required_list: Option<MCMRBRemoveConfiguration>,
        #[rasn(identifier = "mcMRBToModifyRequiredList")]
        pub mc_mrbto_modify_required_list: Option<MCMRBModifyRequiredConfiguration>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MCBearerContextToModifyRequiredIEExtensions>,
    }
    impl MCBearerContextToModifyRequired {
        pub fn new(
            mbs_multicast_f1_ucontext_descriptor: Option<MBSMulticastF1UContextDescriptor>,
            mc_mrbto_remove_required_list: Option<MCMRBRemoveConfiguration>,
            mc_mrbto_modify_required_list: Option<MCMRBModifyRequiredConfiguration>,
            i_e_extensions: Option<MCBearerContextToModifyRequiredIEExtensions>,
        ) -> Self {
            Self {
                mbs_multicast_f1_ucontext_descriptor,
                mc_mrbto_remove_required_list,
                mc_mrbto_modify_required_list,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCBearerContextToModifyResponseIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCBearerContextToModifyResponseIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCBearerContextToModifyResponseIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMCBearerContextToModifyResponseIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMCBearerContextToModifyResponseIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MCBearerContextToModifyResponseIEExtensions(
        pub SequenceOf<AnonymousMCBearerContextToModifyResponseIEExtensions>,
    );
    #[doc = " MCBearerContextToModifyResponse"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCBearerContextToModifyResponse {
        #[rasn(identifier = "mcBearerContextNGU-TNLInfoatNGRANModifyResponse")]
        pub mc_bearer_context_ngu_tnlinfoat_ngranmodify_response:
            Option<MCBearerContextNGUTNLInfoatNGRANModifyResponse>,
        #[rasn(identifier = "mbsMulticastF1UContextDescriptor")]
        pub mbs_multicast_f1_ucontext_descriptor: Option<MBSMulticastF1UContextDescriptor>,
        #[rasn(identifier = "mcMRBModifySetupResponseList")]
        pub mc_mrbmodify_setup_response_list: Option<MCMRBSetupModifyResponseList>,
        #[rasn(identifier = "mcMRBFailedList")]
        pub mc_mrbfailed_list: Option<MCMRBFailedList>,
        #[rasn(identifier = "availableMCMRBConfig")]
        pub available_mcmrbconfig: Option<MCMRBSetupConfiguration>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MCBearerContextToModifyResponseIEExtensions>,
    }
    impl MCBearerContextToModifyResponse {
        pub fn new(
            mc_bearer_context_ngu_tnlinfoat_ngranmodify_response: Option<
                MCBearerContextNGUTNLInfoatNGRANModifyResponse,
            >,
            mbs_multicast_f1_ucontext_descriptor: Option<MBSMulticastF1UContextDescriptor>,
            mc_mrbmodify_setup_response_list: Option<MCMRBSetupModifyResponseList>,
            mc_mrbfailed_list: Option<MCMRBFailedList>,
            available_mcmrbconfig: Option<MCMRBSetupConfiguration>,
            i_e_extensions: Option<MCBearerContextToModifyResponseIEExtensions>,
        ) -> Self {
            Self {
                mc_bearer_context_ngu_tnlinfoat_ngranmodify_response,
                mbs_multicast_f1_ucontext_descriptor,
                mc_mrbmodify_setup_response_list,
                mc_mrbfailed_list,
                available_mcmrbconfig,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCBearerContextToSetupIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCBearerContextToSetupIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCBearerContextToSetupIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMCBearerContextToSetupIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMCBearerContextToSetupIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MCBearerContextToSetupIEExtensions(
        pub SequenceOf<AnonymousMCBearerContextToSetupIEExtensions>,
    );
    #[doc = " MCBearerContextToSetup"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCBearerContextToSetup {
        pub snssai: SNSSAI,
        #[rasn(identifier = "mcMRBToSetupList")]
        pub mc_mrbto_setup_list: Option<MCMRBSetupConfiguration>,
        #[rasn(identifier = "requestedAction")]
        pub requested_action: Option<RequestedAction4AvailNGUTermination>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MCBearerContextToSetupIEExtensions>,
    }
    impl MCBearerContextToSetup {
        pub fn new(
            snssai: SNSSAI,
            mc_mrbto_setup_list: Option<MCMRBSetupConfiguration>,
            requested_action: Option<RequestedAction4AvailNGUTermination>,
            i_e_extensions: Option<MCBearerContextToSetupIEExtensions>,
        ) -> Self {
            Self {
                snssai,
                mc_mrbto_setup_list,
                requested_action,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCBearerContextToSetupResponseIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCBearerContextToSetupResponseIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCBearerContextToSetupResponseIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMCBearerContextToSetupResponseIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMCBearerContextToSetupResponseIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MCBearerContextToSetupResponseIEExtensions(
        pub SequenceOf<AnonymousMCBearerContextToSetupResponseIEExtensions>,
    );
    #[doc = " MCBearerContextToSetupResponse"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCBearerContextToSetupResponse {
        #[rasn(identifier = "mcBearerContextNGU-TNLInfoatNGRAN")]
        pub mc_bearer_context_ngu_tnlinfoat_ngran: Option<MCBearerContextNGUTNLInfoatNGRAN>,
        #[rasn(identifier = "mcMRBSetupResponseList")]
        pub mc_mrbsetup_response_list: Option<MCMRBSetupResponseList>,
        #[rasn(identifier = "mcMRBFailedList")]
        pub mc_mrbfailed_list: Option<MCMRBFailedList>,
        #[rasn(identifier = "availableMCMRBConfig")]
        pub available_mcmrbconfig: Option<MCMRBSetupConfiguration>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MCBearerContextToSetupResponseIEExtensions>,
    }
    impl MCBearerContextToSetupResponse {
        pub fn new(
            mc_bearer_context_ngu_tnlinfoat_ngran: Option<MCBearerContextNGUTNLInfoatNGRAN>,
            mc_mrbsetup_response_list: Option<MCMRBSetupResponseList>,
            mc_mrbfailed_list: Option<MCMRBFailedList>,
            available_mcmrbconfig: Option<MCMRBSetupConfiguration>,
            i_e_extensions: Option<MCBearerContextToSetupResponseIEExtensions>,
        ) -> Self {
            Self {
                mc_bearer_context_ngu_tnlinfoat_ngran,
                mc_mrbsetup_response_list,
                mc_mrbfailed_list,
                available_mcmrbconfig,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct MCForwardingResourceID(pub FixedOctetString<2usize>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCForwardingResourceIndicationIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCForwardingResourceIndicationIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCForwardingResourceIndicationIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMCForwardingResourceIndicationIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMCForwardingResourceIndicationIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MCForwardingResourceIndicationIEExtensions(
        pub SequenceOf<AnonymousMCForwardingResourceIndicationIEExtensions>,
    );
    #[doc = " MCForwardingResourceIndication"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCForwardingResourceIndication {
        #[rasn(identifier = "mcForwardingResourceID")]
        pub mc_forwarding_resource_id: MCForwardingResourceID,
        #[rasn(identifier = "mrbForwardingResourceIndicationList")]
        pub mrb_forwarding_resource_indication_list: Option<MRBForwardingResourceIndicationList>,
        #[rasn(identifier = "mbsSessionAssociatedInformation")]
        pub mbs_session_associated_information: Option<MBSSessionAssociatedInformation>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MCForwardingResourceIndicationIEExtensions>,
    }
    impl MCForwardingResourceIndication {
        pub fn new(
            mc_forwarding_resource_id: MCForwardingResourceID,
            mrb_forwarding_resource_indication_list: Option<MRBForwardingResourceIndicationList>,
            mbs_session_associated_information: Option<MBSSessionAssociatedInformation>,
            i_e_extensions: Option<MCForwardingResourceIndicationIEExtensions>,
        ) -> Self {
            Self {
                mc_forwarding_resource_id,
                mrb_forwarding_resource_indication_list,
                mbs_session_associated_information,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCForwardingResourceReleaseIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCForwardingResourceReleaseIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCForwardingResourceReleaseIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMCForwardingResourceReleaseIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMCForwardingResourceReleaseIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MCForwardingResourceReleaseIEExtensions(
        pub SequenceOf<AnonymousMCForwardingResourceReleaseIEExtensions>,
    );
    #[doc = " MCForwardingResourceRelease"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCForwardingResourceRelease {
        #[rasn(identifier = "mcForwardingResourceID")]
        pub mc_forwarding_resource_id: MCForwardingResourceID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MCForwardingResourceReleaseIEExtensions>,
    }
    impl MCForwardingResourceRelease {
        pub fn new(
            mc_forwarding_resource_id: MCForwardingResourceID,
            i_e_extensions: Option<MCForwardingResourceReleaseIEExtensions>,
        ) -> Self {
            Self {
                mc_forwarding_resource_id,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCForwardingResourceReleaseIndicationIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCForwardingResourceReleaseIndicationIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCForwardingResourceReleaseIndicationIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMCForwardingResourceReleaseIndicationIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMCForwardingResourceReleaseIndicationIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MCForwardingResourceReleaseIndicationIEExtensions(
        pub SequenceOf<AnonymousMCForwardingResourceReleaseIndicationIEExtensions>,
    );
    #[doc = " MCForwardingResourceReleaseIndication"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCForwardingResourceReleaseIndication {
        #[rasn(identifier = "mcForwardingResourceID")]
        pub mc_forwarding_resource_id: MCForwardingResourceID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MCForwardingResourceReleaseIndicationIEExtensions>,
    }
    impl MCForwardingResourceReleaseIndication {
        pub fn new(
            mc_forwarding_resource_id: MCForwardingResourceID,
            i_e_extensions: Option<MCForwardingResourceReleaseIndicationIEExtensions>,
        ) -> Self {
            Self {
                mc_forwarding_resource_id,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCForwardingResourceRequestIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCForwardingResourceRequestIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCForwardingResourceRequestIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMCForwardingResourceRequestIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMCForwardingResourceRequestIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MCForwardingResourceRequestIEExtensions(
        pub SequenceOf<AnonymousMCForwardingResourceRequestIEExtensions>,
    );
    #[doc = " MCForwardingResourceRequest"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCForwardingResourceRequest {
        #[rasn(identifier = "mcForwardingResourceID")]
        pub mc_forwarding_resource_id: MCForwardingResourceID,
        #[rasn(identifier = "mbsAreaSession-ID")]
        pub mbs_area_session_id: Option<MBSAreaSessionID>,
        #[rasn(identifier = "mrbForwardingResourceRequestList")]
        pub mrb_forwarding_resource_request_list: Option<MRBForwardingResourceRequestList>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MCForwardingResourceRequestIEExtensions>,
    }
    impl MCForwardingResourceRequest {
        pub fn new(
            mc_forwarding_resource_id: MCForwardingResourceID,
            mbs_area_session_id: Option<MBSAreaSessionID>,
            mrb_forwarding_resource_request_list: Option<MRBForwardingResourceRequestList>,
            i_e_extensions: Option<MCForwardingResourceRequestIEExtensions>,
        ) -> Self {
            Self {
                mc_forwarding_resource_id,
                mbs_area_session_id,
                mrb_forwarding_resource_request_list,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCForwardingResourceResponseIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCForwardingResourceResponseIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCForwardingResourceResponseIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMCForwardingResourceResponseIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMCForwardingResourceResponseIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MCForwardingResourceResponseIEExtensions(
        pub SequenceOf<AnonymousMCForwardingResourceResponseIEExtensions>,
    );
    #[doc = " MCForwardingResourceResponse"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCForwardingResourceResponse {
        #[rasn(identifier = "mcForwardingResourceID")]
        pub mc_forwarding_resource_id: MCForwardingResourceID,
        #[rasn(identifier = "mrbForwardingResourceResponseList")]
        pub mrb_forwarding_resource_response_list: Option<MRBForwardingResourceResponseList>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MCForwardingResourceResponseIEExtensions>,
    }
    impl MCForwardingResourceResponse {
        pub fn new(
            mc_forwarding_resource_id: MCForwardingResourceID,
            mrb_forwarding_resource_response_list: Option<MRBForwardingResourceResponseList>,
            i_e_extensions: Option<MCForwardingResourceResponseIEExtensions>,
        ) -> Self {
            Self {
                mc_forwarding_resource_id,
                mrb_forwarding_resource_response_list,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct MCMRBFailedList(pub SequenceOf<MCMRBFailedListItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCMRBFailedListItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCMRBFailedListItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCMRBFailedListItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMCMRBFailedListItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMCMRBFailedListItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MCMRBFailedListItemIEExtensions(
        pub SequenceOf<AnonymousMCMRBFailedListItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MCMRBFailedList-Item")]
    #[non_exhaustive]
    pub struct MCMRBFailedListItem {
        #[rasn(identifier = "mrb-ID")]
        pub mrb_id: MRBID,
        pub cause: Cause,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MCMRBFailedListItemIEExtensions>,
    }
    impl MCMRBFailedListItem {
        pub fn new(
            mrb_id: MRBID,
            cause: Cause,
            i_e_extensions: Option<MCMRBFailedListItemIEExtensions>,
        ) -> Self {
            Self {
                mrb_id,
                cause,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct MCMRBModifyConfirmList(pub SequenceOf<MCMRBModifyConfirmListItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCMRBModifyConfirmListItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCMRBModifyConfirmListItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCMRBModifyConfirmListItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMCMRBModifyConfirmListItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMCMRBModifyConfirmListItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MCMRBModifyConfirmListItemIEExtensions(
        pub SequenceOf<AnonymousMCMRBModifyConfirmListItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MCMRBModifyConfirmList-Item")]
    #[non_exhaustive]
    pub struct MCMRBModifyConfirmListItem {
        #[rasn(identifier = "mrb-ID")]
        pub mrb_id: MRBID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MCMRBModifyConfirmListItemIEExtensions>,
    }
    impl MCMRBModifyConfirmListItem {
        pub fn new(
            mrb_id: MRBID,
            i_e_extensions: Option<MCMRBModifyConfirmListItemIEExtensions>,
        ) -> Self {
            Self {
                mrb_id,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct MCMRBModifyRequiredConfiguration(
        pub SequenceOf<MCMRBModifyRequiredConfigurationItem>,
    );
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCMRBModifyRequiredConfigurationItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCMRBModifyRequiredConfigurationItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCMRBModifyRequiredConfigurationItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMCMRBModifyRequiredConfigurationItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMCMRBModifyRequiredConfigurationItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MCMRBModifyRequiredConfigurationItemIEExtensions(
        pub SequenceOf<AnonymousMCMRBModifyRequiredConfigurationItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MCMRBModifyRequiredConfiguration-Item")]
    #[non_exhaustive]
    pub struct MCMRBModifyRequiredConfigurationItem {
        #[rasn(identifier = "mrb-ID")]
        pub mrb_id: MRBID,
        #[rasn(identifier = "mBS-PDCP-COUNT")]
        pub m_bs_pdcp_count: Option<MBSPDCPCOUNT>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MCMRBModifyRequiredConfigurationItemIEExtensions>,
    }
    impl MCMRBModifyRequiredConfigurationItem {
        pub fn new(
            mrb_id: MRBID,
            m_bs_pdcp_count: Option<MBSPDCPCOUNT>,
            i_e_extensions: Option<MCMRBModifyRequiredConfigurationItemIEExtensions>,
        ) -> Self {
            Self {
                mrb_id,
                m_bs_pdcp_count,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct MCMRBRemoveConfiguration(pub SequenceOf<MRBID>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct MCMRBSetupConfiguration(pub SequenceOf<MCMRBSetupConfigurationItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCMRBSetupConfigurationItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCMRBSetupConfigurationItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCMRBSetupConfigurationItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMCMRBSetupConfigurationItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMCMRBSetupConfigurationItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MCMRBSetupConfigurationItemIEExtensions(
        pub SequenceOf<AnonymousMCMRBSetupConfigurationItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MCMRBSetupConfiguration-Item")]
    #[non_exhaustive]
    pub struct MCMRBSetupConfigurationItem {
        #[rasn(identifier = "mrb-ID")]
        pub mrb_id: MRBID,
        #[rasn(identifier = "mbs-pdcp-config")]
        pub mbs_pdcp_config: PDCPConfiguration,
        #[rasn(identifier = "qoS-Flow-QoS-Parameter-List")]
        pub qo_s_flow_qo_s_parameter_list: QoSFlowQoSParameterList,
        #[rasn(identifier = "qoSFlowLevelQoSParameters")]
        pub qo_sflow_level_qo_sparameters: Option<QoSFlowLevelQoSParameters>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MCMRBSetupConfigurationItemIEExtensions>,
    }
    impl MCMRBSetupConfigurationItem {
        pub fn new(
            mrb_id: MRBID,
            mbs_pdcp_config: PDCPConfiguration,
            qo_s_flow_qo_s_parameter_list: QoSFlowQoSParameterList,
            qo_sflow_level_qo_sparameters: Option<QoSFlowLevelQoSParameters>,
            i_e_extensions: Option<MCMRBSetupConfigurationItemIEExtensions>,
        ) -> Self {
            Self {
                mrb_id,
                mbs_pdcp_config,
                qo_s_flow_qo_s_parameter_list,
                qo_sflow_level_qo_sparameters,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct MCMRBSetupModifyConfiguration(pub SequenceOf<MCMRBSetupModifyConfigurationItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCMRBSetupModifyConfigurationItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCMRBSetupModifyConfigurationItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCMRBSetupModifyConfigurationItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMCMRBSetupModifyConfigurationItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMCMRBSetupModifyConfigurationItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MCMRBSetupModifyConfigurationItemIEExtensions(
        pub SequenceOf<AnonymousMCMRBSetupModifyConfigurationItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MCMRBSetupModifyConfiguration-Item")]
    #[non_exhaustive]
    pub struct MCMRBSetupModifyConfigurationItem {
        #[rasn(identifier = "mrb-ID")]
        pub mrb_id: MRBID,
        #[rasn(identifier = "f1uTNLatDU")]
        pub f1u_tnlat_du: Option<MCBearerContextF1UTNLInfoatDU>,
        #[rasn(identifier = "mbs-pdcp-config")]
        pub mbs_pdcp_config: Option<PDCPConfiguration>,
        #[rasn(identifier = "qoS-Flow-QoS-Parameter-List")]
        pub qo_s_flow_qo_s_parameter_list: Option<QoSFlowQoSParameterList>,
        #[rasn(identifier = "mrbQoS")]
        pub mrb_qo_s: Option<QoSFlowLevelQoSParameters>,
        #[rasn(identifier = "mbs-PDCP-COUNT-Req")]
        pub mbs_pdcp_count_req: Option<MBSPDCPCOUNTReq>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MCMRBSetupModifyConfigurationItemIEExtensions>,
    }
    impl MCMRBSetupModifyConfigurationItem {
        pub fn new(
            mrb_id: MRBID,
            f1u_tnlat_du: Option<MCBearerContextF1UTNLInfoatDU>,
            mbs_pdcp_config: Option<PDCPConfiguration>,
            qo_s_flow_qo_s_parameter_list: Option<QoSFlowQoSParameterList>,
            mrb_qo_s: Option<QoSFlowLevelQoSParameters>,
            mbs_pdcp_count_req: Option<MBSPDCPCOUNTReq>,
            i_e_extensions: Option<MCMRBSetupModifyConfigurationItemIEExtensions>,
        ) -> Self {
            Self {
                mrb_id,
                f1u_tnlat_du,
                mbs_pdcp_config,
                qo_s_flow_qo_s_parameter_list,
                mrb_qo_s,
                mbs_pdcp_count_req,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct MCMRBSetupModifyResponseList(pub SequenceOf<MCMRBSetupModifyResponseListItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCMRBSetupModifyResponseListItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCMRBSetupModifyResponseListItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCMRBSetupModifyResponseListItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMCMRBSetupModifyResponseListItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMCMRBSetupModifyResponseListItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MCMRBSetupModifyResponseListItemIEExtensions(
        pub SequenceOf<AnonymousMCMRBSetupModifyResponseListItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MCMRBSetupModifyResponseList-Item")]
    #[non_exhaustive]
    pub struct MCMRBSetupModifyResponseListItem {
        #[rasn(identifier = "mrb-ID")]
        pub mrb_id: MRBID,
        #[rasn(identifier = "qosflow-setup")]
        pub qosflow_setup: Option<QoSFlowList>,
        #[rasn(identifier = "qosflow-failed")]
        pub qosflow_failed: Option<QoSFlowFailedList>,
        #[rasn(identifier = "mcBearerContextF1UTNLInfoatCU")]
        pub mc_bearer_context_f1_utnlinfoat_cu: Option<UPTNLInformation>,
        #[rasn(identifier = "mBS-PDCP-COUNT")]
        pub m_bs_pdcp_count: Option<MBSPDCPCOUNT>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MCMRBSetupModifyResponseListItemIEExtensions>,
    }
    impl MCMRBSetupModifyResponseListItem {
        pub fn new(
            mrb_id: MRBID,
            qosflow_setup: Option<QoSFlowList>,
            qosflow_failed: Option<QoSFlowFailedList>,
            mc_bearer_context_f1_utnlinfoat_cu: Option<UPTNLInformation>,
            m_bs_pdcp_count: Option<MBSPDCPCOUNT>,
            i_e_extensions: Option<MCMRBSetupModifyResponseListItemIEExtensions>,
        ) -> Self {
            Self {
                mrb_id,
                qosflow_setup,
                qosflow_failed,
                mc_bearer_context_f1_utnlinfoat_cu,
                m_bs_pdcp_count,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct MCMRBSetupResponseList(pub SequenceOf<MCMRBSetupResponseListItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCMRBSetupResponseListItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCMRBSetupResponseListItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCMRBSetupResponseListItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMCMRBSetupResponseListItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMCMRBSetupResponseListItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MCMRBSetupResponseListItemIEExtensions(
        pub SequenceOf<AnonymousMCMRBSetupResponseListItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MCMRBSetupResponseList-Item")]
    #[non_exhaustive]
    pub struct MCMRBSetupResponseListItem {
        #[rasn(identifier = "mrb-ID")]
        pub mrb_id: MRBID,
        #[rasn(identifier = "qosflow-setup")]
        pub qosflow_setup: QoSFlowList,
        #[rasn(identifier = "qosflow-failed")]
        pub qosflow_failed: Option<QoSFlowFailedList>,
        #[rasn(identifier = "mBS-PDCP-COUNT")]
        pub m_bs_pdcp_count: Option<MBSPDCPCOUNT>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MCMRBSetupResponseListItemIEExtensions>,
    }
    impl MCMRBSetupResponseListItem {
        pub fn new(
            mrb_id: MRBID,
            qosflow_setup: QoSFlowList,
            qosflow_failed: Option<QoSFlowFailedList>,
            m_bs_pdcp_count: Option<MBSPDCPCOUNT>,
            i_e_extensions: Option<MCMRBSetupResponseListItemIEExtensions>,
        ) -> Self {
            Self {
                mrb_id,
                qosflow_setup,
                qosflow_failed,
                m_bs_pdcp_count,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "MDT-Activation")]
    #[non_exhaustive]
    pub enum MDTActivation {
        #[rasn(identifier = "immediate-MDT-only")]
        immediate_MDT_only = 0,
        #[rasn(identifier = "immediate-MDT-and-Trace")]
        immediate_MDT_and_Trace = 1,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMDTConfigurationIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMDTConfigurationIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMDTConfigurationIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMDTConfigurationIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMDTConfigurationIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MDTConfigurationIEExtensions(pub SequenceOf<AnonymousMDTConfigurationIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MDT-Configuration")]
    #[non_exhaustive]
    pub struct MDTConfiguration {
        #[rasn(identifier = "mdt-Activation")]
        pub mdt_activation: MDTActivation,
        #[rasn(identifier = "mDTMode")]
        pub m_dtmode: MDTMode,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MDTConfigurationIEExtensions>,
    }
    impl MDTConfiguration {
        pub fn new(
            mdt_activation: MDTActivation,
            m_dtmode: MDTMode,
            i_e_extensions: Option<MDTConfigurationIEExtensions>,
        ) -> Self {
            Self {
                mdt_activation,
                m_dtmode,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum MDTModeChoiceExtensionCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct MDTModeChoiceExtension {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: MDTModeChoiceExtensionCriticality,
        pub value: Any,
    }
    impl MDTModeChoiceExtension {
        pub fn new(id: u16, criticality: MDTModeChoiceExtensionCriticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum MDTMode {
        immediateMDT(ImmediateMDT),
        #[rasn(identifier = "choice-extension")]
        choice_extension(MDTModeChoiceExtension),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct MDTPLMNList(pub SequenceOf<PLMNIdentity>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=16"))]
    pub struct MDTPLMNModificationList(pub SequenceOf<PLMNIdentity>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum MDTPollutedMeasurementIndicator {
        iDC = 0,
        #[rasn(identifier = "no-IDC")]
        no_IDC = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "MRB-ID", value("1..=512", extensible))]
    pub struct MRBID(pub Integer);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMRBProgressInformationIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMRBProgressInformationIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMRBProgressInformationIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMRBProgressInformationIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMRBProgressInformationIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MRBProgressInformationIEExtensions(
        pub SequenceOf<AnonymousMRBProgressInformationIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MRB-ProgressInformation")]
    #[non_exhaustive]
    pub struct MRBProgressInformation {
        #[rasn(identifier = "mrb-ProgressInformationSNs")]
        pub mrb_progress_information_sns: MRBProgressInformationSNs,
        #[rasn(identifier = "mrb-ProgressInformationType")]
        pub mrb_progress_information_type: MRBProgressInformationType,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MRBProgressInformationIEExtensions>,
    }
    impl MRBProgressInformation {
        pub fn new(
            mrb_progress_information_sns: MRBProgressInformationSNs,
            mrb_progress_information_type: MRBProgressInformationType,
            i_e_extensions: Option<MRBProgressInformationIEExtensions>,
        ) -> Self {
            Self {
                mrb_progress_information_sns,
                mrb_progress_information_type,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum MRBProgressInformationSNsChoiceExtensionCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct MRBProgressInformationSNsChoiceExtension {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: MRBProgressInformationSNsChoiceExtensionCriticality,
        pub value: Any,
    }
    impl MRBProgressInformationSNsChoiceExtension {
        pub fn new(
            id: u16,
            criticality: MRBProgressInformationSNsChoiceExtensionCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags, identifier = "MRB-ProgressInformationSNs")]
    pub enum MRBProgressInformationSNs {
        #[rasn(value("0..=4095"), identifier = "pdcp-SN12")]
        pdcp_SN12(u16),
        #[rasn(value("0..=262143"), identifier = "pdcp-SN18")]
        pdcp_SN18(u32),
        #[rasn(identifier = "choice-extension")]
        choice_extension(MRBProgressInformationSNsChoiceExtension),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "MRB-ProgressInformationType")]
    #[non_exhaustive]
    pub enum MRBProgressInformationType {
        #[rasn(identifier = "oldest-available")]
        oldest_available = 0,
        #[rasn(identifier = "last-delivered")]
        last_delivered = 1,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMRBForwardingResourceIndicationItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMRBForwardingResourceIndicationItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMRBForwardingResourceIndicationItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMRBForwardingResourceIndicationItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMRBForwardingResourceIndicationItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MRBForwardingResourceIndicationItemIEExtensions(
        pub SequenceOf<AnonymousMRBForwardingResourceIndicationItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MRBForwardingResourceIndication-Item")]
    #[non_exhaustive]
    pub struct MRBForwardingResourceIndicationItem {
        #[rasn(identifier = "mrb-ID")]
        pub mrb_id: MRBID,
        #[rasn(identifier = "mrb-ProgressInformation")]
        pub mrb_progress_information: Option<MRBProgressInformation>,
        #[rasn(identifier = "mrbForwardingAddress")]
        pub mrb_forwarding_address: Option<UPTNLInformation>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MRBForwardingResourceIndicationItemIEExtensions>,
    }
    impl MRBForwardingResourceIndicationItem {
        pub fn new(
            mrb_id: MRBID,
            mrb_progress_information: Option<MRBProgressInformation>,
            mrb_forwarding_address: Option<UPTNLInformation>,
            i_e_extensions: Option<MRBForwardingResourceIndicationItemIEExtensions>,
        ) -> Self {
            Self {
                mrb_id,
                mrb_progress_information,
                mrb_forwarding_address,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=64"))]
    pub struct MRBForwardingResourceIndicationList(
        pub SequenceOf<MRBForwardingResourceIndicationItem>,
    );
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum MRBForwardingResourceRequestItemMrbForwardingAddressRequest {
        request = 0,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMRBForwardingResourceRequestItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMRBForwardingResourceRequestItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMRBForwardingResourceRequestItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMRBForwardingResourceRequestItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMRBForwardingResourceRequestItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MRBForwardingResourceRequestItemIEExtensions(
        pub SequenceOf<AnonymousMRBForwardingResourceRequestItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MRBForwardingResourceRequest-Item")]
    #[non_exhaustive]
    pub struct MRBForwardingResourceRequestItem {
        #[rasn(identifier = "mrb-ID")]
        pub mrb_id: MRBID,
        #[rasn(identifier = "mrbProgressRequestType")]
        pub mrb_progress_request_type: Option<MRBProgressInformationType>,
        #[rasn(identifier = "mrbForwardingAddressRequest")]
        pub mrb_forwarding_address_request:
            Option<MRBForwardingResourceRequestItemMrbForwardingAddressRequest>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MRBForwardingResourceRequestItemIEExtensions>,
    }
    impl MRBForwardingResourceRequestItem {
        pub fn new(
            mrb_id: MRBID,
            mrb_progress_request_type: Option<MRBProgressInformationType>,
            mrb_forwarding_address_request: Option<
                MRBForwardingResourceRequestItemMrbForwardingAddressRequest,
            >,
            i_e_extensions: Option<MRBForwardingResourceRequestItemIEExtensions>,
        ) -> Self {
            Self {
                mrb_id,
                mrb_progress_request_type,
                mrb_forwarding_address_request,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=64"))]
    pub struct MRBForwardingResourceRequestList(pub SequenceOf<MRBForwardingResourceRequestItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMRBForwardingResourceResponseItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMRBForwardingResourceResponseItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMRBForwardingResourceResponseItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMRBForwardingResourceResponseItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMRBForwardingResourceResponseItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MRBForwardingResourceResponseItemIEExtensions(
        pub SequenceOf<AnonymousMRBForwardingResourceResponseItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MRBForwardingResourceResponse-Item")]
    #[non_exhaustive]
    pub struct MRBForwardingResourceResponseItem {
        #[rasn(identifier = "mrb-ID")]
        pub mrb_id: MRBID,
        #[rasn(identifier = "mrb-ProgressInformation")]
        pub mrb_progress_information: Option<MRBProgressInformation>,
        #[rasn(identifier = "mrbForwardingAddress")]
        pub mrb_forwarding_address: Option<UPTNLInformation>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MRBForwardingResourceResponseItemIEExtensions>,
    }
    impl MRBForwardingResourceResponseItem {
        pub fn new(
            mrb_id: MRBID,
            mrb_progress_information: Option<MRBProgressInformation>,
            mrb_forwarding_address: Option<UPTNLInformation>,
            i_e_extensions: Option<MRBForwardingResourceResponseItemIEExtensions>,
        ) -> Self {
            Self {
                mrb_id,
                mrb_progress_information,
                mrb_forwarding_address,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=64"))]
    pub struct MRBForwardingResourceResponseList(pub SequenceOf<MRBForwardingResourceResponseItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMRDCDataUsageReportItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMRDCDataUsageReportItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMRDCDataUsageReportItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMRDCDataUsageReportItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMRDCDataUsageReportItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MRDCDataUsageReportItemIEExtensions(
        pub SequenceOf<AnonymousMRDCDataUsageReportItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MRDC-Data-Usage-Report-Item")]
    #[non_exhaustive]
    pub struct MRDCDataUsageReportItem {
        #[rasn(size("4"), identifier = "startTimeStamp")]
        pub start_time_stamp: OctetString,
        #[rasn(size("4"), identifier = "endTimeStamp")]
        pub end_time_stamp: OctetString,
        #[rasn(value("0..=18446744073709551615"), identifier = "usageCountUL")]
        pub usage_count_ul: u64,
        #[rasn(value("0..=18446744073709551615"), identifier = "usageCountDL")]
        pub usage_count_dl: u64,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MRDCDataUsageReportItemIEExtensions>,
    }
    impl MRDCDataUsageReportItem {
        pub fn new(
            start_time_stamp: OctetString,
            end_time_stamp: OctetString,
            usage_count_ul: u64,
            usage_count_dl: u64,
            i_e_extensions: Option<MRDCDataUsageReportItemIEExtensions>,
        ) -> Self {
            Self {
                start_time_stamp,
                end_time_stamp,
                usage_count_ul,
                usage_count_dl,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMRDCUsageInformationIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMRDCUsageInformationIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMRDCUsageInformationIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMRDCUsageInformationIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMRDCUsageInformationIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MRDCUsageInformationIEExtensions(
        pub SequenceOf<AnonymousMRDCUsageInformationIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MRDC-Usage-Information")]
    #[non_exhaustive]
    pub struct MRDCUsageInformation {
        #[rasn(identifier = "data-Usage-per-PDU-Session-Report")]
        pub data_usage_per_pdu_session_report: Option<DataUsagePerPDUSessionReport>,
        #[rasn(identifier = "data-Usage-per-QoS-Flow-List")]
        pub data_usage_per_qo_s_flow_list: Option<DataUsagePerQoSFlowList>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MRDCUsageInformationIEExtensions>,
    }
    impl MRDCUsageInformation {
        pub fn new(
            data_usage_per_pdu_session_report: Option<DataUsagePerPDUSessionReport>,
            data_usage_per_qo_s_flow_list: Option<DataUsagePerQoSFlowList>,
            i_e_extensions: Option<MRDCUsageInformationIEExtensions>,
        ) -> Self {
            Self {
                data_usage_per_pdu_session_report,
                data_usage_per_qo_s_flow_list,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "MT-SDT-Data-Size",
        value("1..=96000", extensible)
    )]
    pub struct MTSDTDataSize(pub Integer);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMTSDTInformationIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMTSDTInformationIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMTSDTInformationIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMTSDTInformationIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMTSDTInformationIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MTSDTInformationIEExtensions(pub SequenceOf<AnonymousMTSDTInformationIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MT-SDT-Information")]
    pub struct MTSDTInformation {
        #[rasn(identifier = "mT-SDT-Data-Size")]
        pub m_t_sdt_data_size: MTSDTDataSize,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MTSDTInformationIEExtensions>,
    }
    impl MTSDTInformation {
        pub fn new(
            m_t_sdt_data_size: MTSDTDataSize,
            i_e_extensions: Option<MTSDTInformationIEExtensions>,
        ) -> Self {
            Self {
                m_t_sdt_data_size,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "MT-SDT-Information-Request")]
    #[non_exhaustive]
    pub enum MTSDTInformationRequest {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=32767", extensible))]
    pub struct MaxCIDEHCDL(pub Integer);
    #[doc = " M"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=4095", extensible))]
    pub struct MaxDataBurstVolume(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum MaxIPrate {
        bitrate64kbs = 0,
        #[rasn(identifier = "max-UErate")]
        max_UErate = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=1000", extensible))]
    pub struct MaxPacketLossRate(pub Integer);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMaximumIPdatarateIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMaximumIPdatarateIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMaximumIPdatarateIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousMaximumIPdatarateIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousMaximumIPdatarateIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct MaximumIPdatarateIEExtensions(
        pub SequenceOf<AnonymousMaximumIPdatarateIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MaximumIPdatarate {
        #[rasn(identifier = "maxIPrate")]
        pub max_iprate: MaxIPrate,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<MaximumIPdatarateIEExtensions>,
    }
    impl MaximumIPdatarate {
        pub fn new(
            max_iprate: MaxIPrate,
            i_e_extensions: Option<MaximumIPdatarateIEExtensions>,
        ) -> Self {
            Self {
                max_iprate,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct MeasurementsToActivate(pub FixedBitString<8usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct MulticastF1UContextReferenceE1(pub FixedOctetString<4usize>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousN6JitterInformationIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousN6JitterInformationIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousN6JitterInformationIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousN6JitterInformationIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousN6JitterInformationIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct N6JitterInformationIEExtensions(
        pub SequenceOf<AnonymousN6JitterInformationIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct N6JitterInformation {
        #[rasn(value("-127..=127"), identifier = "n6JitterLowerBound")]
        pub n6_jitter_lower_bound: i8,
        #[rasn(value("-127..=127"), identifier = "n6JitterUpperBound")]
        pub n6_jitter_upper_bound: i8,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<N6JitterInformationIEExtensions>,
    }
    impl N6JitterInformation {
        pub fn new(
            n6_jitter_lower_bound: i8,
            n6_jitter_upper_bound: i8,
            i_e_extensions: Option<N6JitterInformationIEExtensions>,
        ) -> Self {
            Self {
                n6_jitter_lower_bound,
                n6_jitter_upper_bound,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousNGRANQoSSupportItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousNGRANQoSSupportItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousNGRANQoSSupportItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousNGRANQoSSupportItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousNGRANQoSSupportItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct NGRANQoSSupportItemIEExtensions(
        pub SequenceOf<AnonymousNGRANQoSSupportItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "NG-RAN-QoS-Support-Item")]
    pub struct NGRANQoSSupportItem {
        #[rasn(identifier = "non-Dynamic5QIDescriptor")]
        pub non_dynamic5_qidescriptor: NonDynamic5QIDescriptor,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<NGRANQoSSupportItemIEExtensions>,
    }
    impl NGRANQoSSupportItem {
        pub fn new(
            non_dynamic5_qidescriptor: NonDynamic5QIDescriptor,
            i_e_extensions: Option<NGRANQoSSupportItemIEExtensions>,
        ) -> Self {
            Self {
                non_dynamic5_qidescriptor,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"), identifier = "NG-RAN-QoS-Support-List")]
    pub struct NGRANQoSSupportList(pub SequenceOf<NGRANQoSSupportItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousNGRANAllocationAndRetentionPriorityIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousNGRANAllocationAndRetentionPriorityIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousNGRANAllocationAndRetentionPriorityIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousNGRANAllocationAndRetentionPriorityIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousNGRANAllocationAndRetentionPriorityIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct NGRANAllocationAndRetentionPriorityIEExtensions(
        pub SequenceOf<AnonymousNGRANAllocationAndRetentionPriorityIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct NGRANAllocationAndRetentionPriority {
        #[rasn(identifier = "priorityLevel")]
        pub priority_level: PriorityLevel,
        #[rasn(identifier = "pre-emptionCapability")]
        pub pre_emption_capability: PreEmptionCapability,
        #[rasn(identifier = "pre-emptionVulnerability")]
        pub pre_emption_vulnerability: PreEmptionVulnerability,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<NGRANAllocationAndRetentionPriorityIEExtensions>,
    }
    impl NGRANAllocationAndRetentionPriority {
        pub fn new(
            priority_level: PriorityLevel,
            pre_emption_capability: PreEmptionCapability,
            pre_emption_vulnerability: PreEmptionVulnerability,
            i_e_extensions: Option<NGRANAllocationAndRetentionPriorityIEExtensions>,
        ) -> Self {
            Self {
                priority_level,
                pre_emption_capability,
                pre_emption_vulnerability,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct NID(pub FixedBitString<44usize>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum NPNContextInfoChoiceExtensionCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct NPNContextInfoChoiceExtension {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: NPNContextInfoChoiceExtensionCriticality,
        pub value: Any,
    }
    impl NPNContextInfoChoiceExtension {
        pub fn new(
            id: u16,
            criticality: NPNContextInfoChoiceExtensionCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum NPNContextInfo {
        sNPN(NPNContextInfoSNPN),
        #[rasn(identifier = "choice-extension")]
        choice_extension(NPNContextInfoChoiceExtension),
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousNPNContextInfoSNPNIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousNPNContextInfoSNPNIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousNPNContextInfoSNPNIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousNPNContextInfoSNPNIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousNPNContextInfoSNPNIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct NPNContextInfoSNPNIEExtensions(
        pub SequenceOf<AnonymousNPNContextInfoSNPNIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "NPNContextInfo-SNPN")]
    pub struct NPNContextInfoSNPN {
        #[rasn(identifier = "nID")]
        pub n_id: NID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<NPNContextInfoSNPNIEExtensions>,
    }
    impl NPNContextInfoSNPN {
        pub fn new(n_id: NID, i_e_extensions: Option<NPNContextInfoSNPNIEExtensions>) -> Self {
            Self {
                n_id,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum NPNSupportInfoChoiceExtensionCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct NPNSupportInfoChoiceExtension {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: NPNSupportInfoChoiceExtensionCriticality,
        pub value: Any,
    }
    impl NPNSupportInfoChoiceExtension {
        pub fn new(
            id: u16,
            criticality: NPNSupportInfoChoiceExtensionCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum NPNSupportInfo {
        sNPN(NPNSupportInfoSNPN),
        #[rasn(identifier = "choice-extension")]
        choice_extension(NPNSupportInfoChoiceExtension),
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousNPNSupportInfoSNPNIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousNPNSupportInfoSNPNIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousNPNSupportInfoSNPNIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousNPNSupportInfoSNPNIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousNPNSupportInfoSNPNIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct NPNSupportInfoSNPNIEExtensions(
        pub SequenceOf<AnonymousNPNSupportInfoSNPNIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "NPNSupportInfo-SNPN")]
    pub struct NPNSupportInfoSNPN {
        #[rasn(identifier = "nID")]
        pub n_id: NID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<NPNSupportInfoSNPNIEExtensions>,
    }
    impl NPNSupportInfoSNPN {
        pub fn new(n_id: NID, i_e_extensions: Option<NPNSupportInfoSNPNIEExtensions>) -> Self {
            Self {
                n_id,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousNRCGIIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousNRCGIIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousNRCGIIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousNRCGIIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousNRCGIIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct NRCGIIEExtensions(pub SequenceOf<AnonymousNRCGIIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "NR-CGI")]
    pub struct NRCGI {
        #[rasn(identifier = "pLMN-Identity")]
        pub p_lmn_identity: PLMNIdentity,
        #[rasn(identifier = "nR-Cell-Identity")]
        pub n_r_cell_identity: NRCellIdentity,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<NRCGIIEExtensions>,
    }
    impl NRCGI {
        pub fn new(
            p_lmn_identity: PLMNIdentity,
            n_r_cell_identity: NRCellIdentity,
            i_e_extensions: Option<NRCGIIEExtensions>,
        ) -> Self {
            Self {
                p_lmn_identity,
                n_r_cell_identity,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousNRCGISupportItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousNRCGISupportItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousNRCGISupportItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousNRCGISupportItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousNRCGISupportItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct NRCGISupportItemIEExtensions(pub SequenceOf<AnonymousNRCGISupportItemIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "NR-CGI-Support-Item")]
    pub struct NRCGISupportItem {
        #[rasn(identifier = "nR-CGI")]
        pub n_r_cgi: NRCGI,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<NRCGISupportItemIEExtensions>,
    }
    impl NRCGISupportItem {
        pub fn new(n_r_cgi: NRCGI, i_e_extensions: Option<NRCGISupportItemIEExtensions>) -> Self {
            Self {
                n_r_cgi,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=512"), identifier = "NR-CGI-Support-List")]
    pub struct NRCGISupportList(pub SequenceOf<NRCGISupportItem>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "NR-Cell-Identity")]
    pub struct NRCellIdentity(pub FixedBitString<36usize>);
    #[doc = " N"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=256", extensible))]
    pub struct NetworkInstance(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "New-UL-TNL-Information-Required")]
    #[non_exhaustive]
    pub enum NewULTNLInformationRequired {
        required = 0,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousNonDynamic5QIDescriptorIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousNonDynamic5QIDescriptorIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousNonDynamic5QIDescriptorIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousNonDynamic5QIDescriptorIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousNonDynamic5QIDescriptorIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct NonDynamic5QIDescriptorIEExtensions(
        pub SequenceOf<AnonymousNonDynamic5QIDescriptorIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Non-Dynamic5QIDescriptor")]
    pub struct NonDynamic5QIDescriptor {
        #[rasn(value("0..=255", extensible), identifier = "fiveQI")]
        pub five_qi: Integer,
        #[rasn(identifier = "qoSPriorityLevel")]
        pub qo_spriority_level: Option<QoSPriorityLevel>,
        #[rasn(identifier = "averagingWindow")]
        pub averaging_window: Option<AveragingWindow>,
        #[rasn(identifier = "maxDataBurstVolume")]
        pub max_data_burst_volume: Option<MaxDataBurstVolume>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<NonDynamic5QIDescriptorIEExtensions>,
    }
    impl NonDynamic5QIDescriptor {
        pub fn new(
            five_qi: Integer,
            qo_spriority_level: Option<QoSPriorityLevel>,
            averaging_window: Option<AveragingWindow>,
            max_data_burst_volume: Option<MaxDataBurstVolume>,
            i_e_extensions: Option<NonDynamic5QIDescriptorIEExtensions>,
        ) -> Self {
            Self {
                five_qi,
                qo_spriority_level,
                averaging_window,
                max_data_burst_volume,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Number-of-tunnels", value("1..=4", extensible))]
    pub struct NumberOfTunnels(pub Integer);
    #[doc = " O"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum OutOfOrderDelivery {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "PDCP-COUNT-Reset")]
    #[non_exhaustive]
    pub enum PDCPCOUNTReset {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousPDCPConfigurationIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPDCPConfigurationIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousPDCPConfigurationIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousPDCPConfigurationIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousPDCPConfigurationIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct PDCPConfigurationIEExtensions(
        pub SequenceOf<AnonymousPDCPConfigurationIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "PDCP-Configuration")]
    #[non_exhaustive]
    pub struct PDCPConfiguration {
        #[rasn(identifier = "pDCP-SN-Size-UL")]
        pub p_dcp_sn_size_ul: PDCPSNSize,
        #[rasn(identifier = "pDCP-SN-Size-DL")]
        pub p_dcp_sn_size_dl: PDCPSNSize,
        #[rasn(identifier = "rLC-Mode")]
        pub r_lc_mode: RLCMode,
        #[rasn(identifier = "rOHC-Parameters")]
        pub r_ohc_parameters: Option<ROHCParameters>,
        #[rasn(identifier = "t-ReorderingTimer")]
        pub t_reordering_timer: Option<TReorderingTimer>,
        #[rasn(identifier = "discardTimer")]
        pub discard_timer: Option<DiscardTimer>,
        #[rasn(identifier = "uLDataSplitThreshold")]
        pub u_ldata_split_threshold: Option<ULDataSplitThreshold>,
        #[rasn(identifier = "pDCP-Duplication")]
        pub p_dcp_duplication: Option<PDCPDuplication>,
        #[rasn(identifier = "pDCP-Reestablishment")]
        pub p_dcp_reestablishment: Option<PDCPReestablishment>,
        #[rasn(identifier = "pDCP-DataRecovery")]
        pub p_dcp_data_recovery: Option<PDCPDataRecovery>,
        #[rasn(identifier = "duplication-Activation")]
        pub duplication_activation: Option<DuplicationActivation>,
        #[rasn(identifier = "outOfOrderDelivery")]
        pub out_of_order_delivery: Option<OutOfOrderDelivery>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<PDCPConfigurationIEExtensions>,
    }
    impl PDCPConfiguration {
        pub fn new(
            p_dcp_sn_size_ul: PDCPSNSize,
            p_dcp_sn_size_dl: PDCPSNSize,
            r_lc_mode: RLCMode,
            r_ohc_parameters: Option<ROHCParameters>,
            t_reordering_timer: Option<TReorderingTimer>,
            discard_timer: Option<DiscardTimer>,
            u_ldata_split_threshold: Option<ULDataSplitThreshold>,
            p_dcp_duplication: Option<PDCPDuplication>,
            p_dcp_reestablishment: Option<PDCPReestablishment>,
            p_dcp_data_recovery: Option<PDCPDataRecovery>,
            duplication_activation: Option<DuplicationActivation>,
            out_of_order_delivery: Option<OutOfOrderDelivery>,
            i_e_extensions: Option<PDCPConfigurationIEExtensions>,
        ) -> Self {
            Self {
                p_dcp_sn_size_ul,
                p_dcp_sn_size_dl,
                r_lc_mode,
                r_ohc_parameters,
                t_reordering_timer,
                discard_timer,
                u_ldata_split_threshold,
                p_dcp_duplication,
                p_dcp_reestablishment,
                p_dcp_data_recovery,
                duplication_activation,
                out_of_order_delivery,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousPDCPCountIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPDCPCountIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousPDCPCountIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousPDCPCountIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousPDCPCountIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct PDCPCountIEExtensions(pub SequenceOf<AnonymousPDCPCountIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "PDCP-Count")]
    #[non_exhaustive]
    pub struct PDCPCount {
        #[rasn(identifier = "pDCP-SN")]
        pub p_dcp_sn: PDCPSN,
        #[rasn(identifier = "hFN")]
        pub h_fn: HFN,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<PDCPCountIEExtensions>,
    }
    impl PDCPCount {
        pub fn new(
            p_dcp_sn: PDCPSN,
            h_fn: HFN,
            i_e_extensions: Option<PDCPCountIEExtensions>,
        ) -> Self {
            Self {
                p_dcp_sn,
                h_fn,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "PDCP-DataRecovery")]
    #[non_exhaustive]
    pub enum PDCPDataRecovery {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "PDCP-Duplication")]
    #[non_exhaustive]
    pub enum PDCPDuplication {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "PDCP-Reestablishment")]
    #[non_exhaustive]
    pub enum PDCPReestablishment {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "PDCP-SN", value("0..=262143"))]
    pub struct PDCPSN(pub u32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "PDCP-SN-Size")]
    #[non_exhaustive]
    pub enum PDCPSNSize {
        #[rasn(identifier = "s-12")]
        s_12 = 0,
        #[rasn(identifier = "s-18")]
        s_18 = 1,
        #[rasn(extension_addition, identifier = "s-7")]
        s_7 = 2,
        #[rasn(extension_addition, identifier = "s-15")]
        s_15 = 3,
        #[rasn(extension_addition, identifier = "s-16")]
        s_16 = 4,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousPDCPSNStatusInformationIEExtensionCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPDCPSNStatusInformationIEExtension {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousPDCPSNStatusInformationIEExtensionCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousPDCPSNStatusInformationIEExtension {
        pub fn new(
            id: u16,
            criticality: AnonymousPDCPSNStatusInformationIEExtensionCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct PDCPSNStatusInformationIEExtension(
        pub SequenceOf<AnonymousPDCPSNStatusInformationIEExtension>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "PDCP-SN-Status-Information")]
    #[non_exhaustive]
    pub struct PDCPSNStatusInformation {
        #[rasn(identifier = "pdcpStatusTransfer-UL")]
        pub pdcp_status_transfer_ul: DRBBStatusTransfer,
        #[rasn(identifier = "pdcpStatusTransfer-DL")]
        pub pdcp_status_transfer_dl: PDCPCount,
        #[rasn(identifier = "iE-Extension")]
        pub i_e_extension: Option<PDCPSNStatusInformationIEExtension>,
    }
    impl PDCPSNStatusInformation {
        pub fn new(
            pdcp_status_transfer_ul: DRBBStatusTransfer,
            pdcp_status_transfer_dl: PDCPCount,
            i_e_extension: Option<PDCPSNStatusInformationIEExtension>,
        ) -> Self {
            Self {
                pdcp_status_transfer_ul,
                pdcp_status_transfer_dl,
                i_e_extension,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "PDCP-SN-Status-Request")]
    #[non_exhaustive]
    pub enum PDCPSNStatusRequest {
        requested = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "PDCP-StatusReportIndication")]
    #[non_exhaustive]
    pub enum PDCPStatusReportIndication {
        downlink = 0,
        uplink = 1,
        both = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PDCPSNGapReport {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "PDU-Session-ID", value("0..=255"))]
    pub struct PDUSessionID(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "PDU-Session-Resource-Activity")]
    #[non_exhaustive]
    pub enum PDUSessionResourceActivity {
        active = 0,
        #[rasn(identifier = "not-active")]
        not_active = 1,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousPDUSessionResourceActivityItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPDUSessionResourceActivityItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousPDUSessionResourceActivityItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousPDUSessionResourceActivityItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousPDUSessionResourceActivityItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct PDUSessionResourceActivityItemIEExtensions(
        pub SequenceOf<AnonymousPDUSessionResourceActivityItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "PDU-Session-Resource-Activity-Item")]
    #[non_exhaustive]
    pub struct PDUSessionResourceActivityItem {
        #[rasn(identifier = "pDU-Session-ID")]
        pub p_du_session_id: PDUSessionID,
        #[rasn(identifier = "pDU-Session-Resource-Activity")]
        pub p_du_session_resource_activity: PDUSessionResourceActivity,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<PDUSessionResourceActivityItemIEExtensions>,
    }
    impl PDUSessionResourceActivityItem {
        pub fn new(
            p_du_session_id: PDUSessionID,
            p_du_session_resource_activity: PDUSessionResourceActivity,
            i_e_extensions: Option<PDUSessionResourceActivityItemIEExtensions>,
        ) -> Self {
            Self {
                p_du_session_id,
                p_du_session_resource_activity,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "PDU-Session-Resource-Activity-List"
    )]
    pub struct PDUSessionResourceActivityList(pub SequenceOf<PDUSessionResourceActivityItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousPDUSessionResourceConfirmModifiedItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPDUSessionResourceConfirmModifiedItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousPDUSessionResourceConfirmModifiedItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousPDUSessionResourceConfirmModifiedItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousPDUSessionResourceConfirmModifiedItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct PDUSessionResourceConfirmModifiedItemIEExtensions(
        pub SequenceOf<AnonymousPDUSessionResourceConfirmModifiedItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "PDU-Session-Resource-Confirm-Modified-Item"
    )]
    #[non_exhaustive]
    pub struct PDUSessionResourceConfirmModifiedItem {
        #[rasn(identifier = "pDU-Session-ID")]
        pub p_du_session_id: PDUSessionID,
        #[rasn(identifier = "dRB-Confirm-Modified-List-NG-RAN")]
        pub d_rb_confirm_modified_list_ng_ran: Option<DRBConfirmModifiedListNGRAN>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<PDUSessionResourceConfirmModifiedItemIEExtensions>,
    }
    impl PDUSessionResourceConfirmModifiedItem {
        pub fn new(
            p_du_session_id: PDUSessionID,
            d_rb_confirm_modified_list_ng_ran: Option<DRBConfirmModifiedListNGRAN>,
            i_e_extensions: Option<PDUSessionResourceConfirmModifiedItemIEExtensions>,
        ) -> Self {
            Self {
                p_du_session_id,
                d_rb_confirm_modified_list_ng_ran,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "PDU-Session-Resource-Confirm-Modified-List"
    )]
    pub struct PDUSessionResourceConfirmModifiedList(
        pub SequenceOf<PDUSessionResourceConfirmModifiedItem>,
    );
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousPDUSessionResourceDataUsageItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPDUSessionResourceDataUsageItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousPDUSessionResourceDataUsageItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousPDUSessionResourceDataUsageItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousPDUSessionResourceDataUsageItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct PDUSessionResourceDataUsageItemIEExtensions(
        pub SequenceOf<AnonymousPDUSessionResourceDataUsageItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "PDU-Session-Resource-Data-Usage-Item")]
    #[non_exhaustive]
    pub struct PDUSessionResourceDataUsageItem {
        #[rasn(identifier = "pDU-Session-ID")]
        pub p_du_session_id: PDUSessionID,
        #[rasn(identifier = "mRDC-Usage-Information")]
        pub m_rdc_usage_information: MRDCUsageInformation,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<PDUSessionResourceDataUsageItemIEExtensions>,
    }
    impl PDUSessionResourceDataUsageItem {
        pub fn new(
            p_du_session_id: PDUSessionID,
            m_rdc_usage_information: MRDCUsageInformation,
            i_e_extensions: Option<PDUSessionResourceDataUsageItemIEExtensions>,
        ) -> Self {
            Self {
                p_du_session_id,
                m_rdc_usage_information,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "PDU-Session-Resource-Data-Usage-List"
    )]
    pub struct PDUSessionResourceDataUsageList(pub SequenceOf<PDUSessionResourceDataUsageItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousPDUSessionResourceFailedItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPDUSessionResourceFailedItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousPDUSessionResourceFailedItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousPDUSessionResourceFailedItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousPDUSessionResourceFailedItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct PDUSessionResourceFailedItemIEExtensions(
        pub SequenceOf<AnonymousPDUSessionResourceFailedItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "PDU-Session-Resource-Failed-Item")]
    #[non_exhaustive]
    pub struct PDUSessionResourceFailedItem {
        #[rasn(identifier = "pDU-Session-ID")]
        pub p_du_session_id: PDUSessionID,
        pub cause: Cause,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<PDUSessionResourceFailedItemIEExtensions>,
    }
    impl PDUSessionResourceFailedItem {
        pub fn new(
            p_du_session_id: PDUSessionID,
            cause: Cause,
            i_e_extensions: Option<PDUSessionResourceFailedItemIEExtensions>,
        ) -> Self {
            Self {
                p_du_session_id,
                cause,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "PDU-Session-Resource-Failed-List"
    )]
    pub struct PDUSessionResourceFailedList(pub SequenceOf<PDUSessionResourceFailedItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousPDUSessionResourceFailedModItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPDUSessionResourceFailedModItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousPDUSessionResourceFailedModItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousPDUSessionResourceFailedModItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousPDUSessionResourceFailedModItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct PDUSessionResourceFailedModItemIEExtensions(
        pub SequenceOf<AnonymousPDUSessionResourceFailedModItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "PDU-Session-Resource-Failed-Mod-Item")]
    #[non_exhaustive]
    pub struct PDUSessionResourceFailedModItem {
        #[rasn(identifier = "pDU-Session-ID")]
        pub p_du_session_id: PDUSessionID,
        pub cause: Cause,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<PDUSessionResourceFailedModItemIEExtensions>,
    }
    impl PDUSessionResourceFailedModItem {
        pub fn new(
            p_du_session_id: PDUSessionID,
            cause: Cause,
            i_e_extensions: Option<PDUSessionResourceFailedModItemIEExtensions>,
        ) -> Self {
            Self {
                p_du_session_id,
                cause,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "PDU-Session-Resource-Failed-Mod-List"
    )]
    pub struct PDUSessionResourceFailedModList(pub SequenceOf<PDUSessionResourceFailedModItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousPDUSessionResourceFailedToModifyItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPDUSessionResourceFailedToModifyItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousPDUSessionResourceFailedToModifyItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousPDUSessionResourceFailedToModifyItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousPDUSessionResourceFailedToModifyItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct PDUSessionResourceFailedToModifyItemIEExtensions(
        pub SequenceOf<AnonymousPDUSessionResourceFailedToModifyItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "PDU-Session-Resource-Failed-To-Modify-Item"
    )]
    #[non_exhaustive]
    pub struct PDUSessionResourceFailedToModifyItem {
        #[rasn(identifier = "pDU-Session-ID")]
        pub p_du_session_id: PDUSessionID,
        pub cause: Cause,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<PDUSessionResourceFailedToModifyItemIEExtensions>,
    }
    impl PDUSessionResourceFailedToModifyItem {
        pub fn new(
            p_du_session_id: PDUSessionID,
            cause: Cause,
            i_e_extensions: Option<PDUSessionResourceFailedToModifyItemIEExtensions>,
        ) -> Self {
            Self {
                p_du_session_id,
                cause,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "PDU-Session-Resource-Failed-To-Modify-List"
    )]
    pub struct PDUSessionResourceFailedToModifyList(
        pub SequenceOf<PDUSessionResourceFailedToModifyItem>,
    );
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousPDUSessionResourceModifiedItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPDUSessionResourceModifiedItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousPDUSessionResourceModifiedItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousPDUSessionResourceModifiedItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousPDUSessionResourceModifiedItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct PDUSessionResourceModifiedItemIEExtensions(
        pub SequenceOf<AnonymousPDUSessionResourceModifiedItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "PDU-Session-Resource-Modified-Item")]
    #[non_exhaustive]
    pub struct PDUSessionResourceModifiedItem {
        #[rasn(identifier = "pDU-Session-ID")]
        pub p_du_session_id: PDUSessionID,
        #[rasn(identifier = "nG-DL-UP-TNL-Information")]
        pub n_g_dl_up_tnl_information: Option<UPTNLInformation>,
        #[rasn(identifier = "securityResult")]
        pub security_result: Option<SecurityResult>,
        #[rasn(identifier = "pDU-Session-Data-Forwarding-Information-Response")]
        pub p_du_session_data_forwarding_information_response: Option<DataForwardingInformation>,
        #[rasn(identifier = "dRB-Setup-List-NG-RAN")]
        pub d_rb_setup_list_ng_ran: Option<DRBSetupListNGRAN>,
        #[rasn(identifier = "dRB-Failed-List-NG-RAN")]
        pub d_rb_failed_list_ng_ran: Option<DRBFailedListNGRAN>,
        #[rasn(identifier = "dRB-Modified-List-NG-RAN")]
        pub d_rb_modified_list_ng_ran: Option<DRBModifiedListNGRAN>,
        #[rasn(identifier = "dRB-Failed-To-Modify-List-NG-RAN")]
        pub d_rb_failed_to_modify_list_ng_ran: Option<DRBFailedToModifyListNGRAN>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<PDUSessionResourceModifiedItemIEExtensions>,
    }
    impl PDUSessionResourceModifiedItem {
        pub fn new(
            p_du_session_id: PDUSessionID,
            n_g_dl_up_tnl_information: Option<UPTNLInformation>,
            security_result: Option<SecurityResult>,
            p_du_session_data_forwarding_information_response: Option<DataForwardingInformation>,
            d_rb_setup_list_ng_ran: Option<DRBSetupListNGRAN>,
            d_rb_failed_list_ng_ran: Option<DRBFailedListNGRAN>,
            d_rb_modified_list_ng_ran: Option<DRBModifiedListNGRAN>,
            d_rb_failed_to_modify_list_ng_ran: Option<DRBFailedToModifyListNGRAN>,
            i_e_extensions: Option<PDUSessionResourceModifiedItemIEExtensions>,
        ) -> Self {
            Self {
                p_du_session_id,
                n_g_dl_up_tnl_information,
                security_result,
                p_du_session_data_forwarding_information_response,
                d_rb_setup_list_ng_ran,
                d_rb_failed_list_ng_ran,
                d_rb_modified_list_ng_ran,
                d_rb_failed_to_modify_list_ng_ran,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "PDU-Session-Resource-Modified-List"
    )]
    pub struct PDUSessionResourceModifiedList(pub SequenceOf<PDUSessionResourceModifiedItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousPDUSessionResourceRequiredToModifyItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPDUSessionResourceRequiredToModifyItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousPDUSessionResourceRequiredToModifyItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousPDUSessionResourceRequiredToModifyItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousPDUSessionResourceRequiredToModifyItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct PDUSessionResourceRequiredToModifyItemIEExtensions(
        pub SequenceOf<AnonymousPDUSessionResourceRequiredToModifyItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "PDU-Session-Resource-Required-To-Modify-Item"
    )]
    #[non_exhaustive]
    pub struct PDUSessionResourceRequiredToModifyItem {
        #[rasn(identifier = "pDU-Session-ID")]
        pub p_du_session_id: PDUSessionID,
        #[rasn(identifier = "nG-DL-UP-TNL-Information")]
        pub n_g_dl_up_tnl_information: Option<UPTNLInformation>,
        #[rasn(identifier = "dRB-Required-To-Modify-List-NG-RAN")]
        pub d_rb_required_to_modify_list_ng_ran: Option<DRBRequiredToModifyListNGRAN>,
        #[rasn(identifier = "dRB-Required-To-Remove-List-NG-RAN")]
        pub d_rb_required_to_remove_list_ng_ran: Option<DRBRequiredToRemoveListNGRAN>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<PDUSessionResourceRequiredToModifyItemIEExtensions>,
    }
    impl PDUSessionResourceRequiredToModifyItem {
        pub fn new(
            p_du_session_id: PDUSessionID,
            n_g_dl_up_tnl_information: Option<UPTNLInformation>,
            d_rb_required_to_modify_list_ng_ran: Option<DRBRequiredToModifyListNGRAN>,
            d_rb_required_to_remove_list_ng_ran: Option<DRBRequiredToRemoveListNGRAN>,
            i_e_extensions: Option<PDUSessionResourceRequiredToModifyItemIEExtensions>,
        ) -> Self {
            Self {
                p_du_session_id,
                n_g_dl_up_tnl_information,
                d_rb_required_to_modify_list_ng_ran,
                d_rb_required_to_remove_list_ng_ran,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "PDU-Session-Resource-Required-To-Modify-List"
    )]
    pub struct PDUSessionResourceRequiredToModifyList(
        pub SequenceOf<PDUSessionResourceRequiredToModifyItem>,
    );
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PDUSessionResourceSetupItemNGDLUPUnchanged {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousPDUSessionResourceSetupItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPDUSessionResourceSetupItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousPDUSessionResourceSetupItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousPDUSessionResourceSetupItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousPDUSessionResourceSetupItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct PDUSessionResourceSetupItemIEExtensions(
        pub SequenceOf<AnonymousPDUSessionResourceSetupItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "PDU-Session-Resource-Setup-Item")]
    #[non_exhaustive]
    pub struct PDUSessionResourceSetupItem {
        #[rasn(identifier = "pDU-Session-ID")]
        pub p_du_session_id: PDUSessionID,
        #[rasn(identifier = "securityResult")]
        pub security_result: Option<SecurityResult>,
        #[rasn(identifier = "nG-DL-UP-TNL-Information")]
        pub n_g_dl_up_tnl_information: UPTNLInformation,
        #[rasn(identifier = "pDU-Session-Data-Forwarding-Information-Response")]
        pub p_du_session_data_forwarding_information_response: Option<DataForwardingInformation>,
        #[rasn(identifier = "nG-DL-UP-Unchanged")]
        pub n_g_dl_up_unchanged: Option<PDUSessionResourceSetupItemNGDLUPUnchanged>,
        #[rasn(identifier = "dRB-Setup-List-NG-RAN")]
        pub d_rb_setup_list_ng_ran: DRBSetupListNGRAN,
        #[rasn(identifier = "dRB-Failed-List-NG-RAN")]
        pub d_rb_failed_list_ng_ran: Option<DRBFailedListNGRAN>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<PDUSessionResourceSetupItemIEExtensions>,
    }
    impl PDUSessionResourceSetupItem {
        pub fn new(
            p_du_session_id: PDUSessionID,
            security_result: Option<SecurityResult>,
            n_g_dl_up_tnl_information: UPTNLInformation,
            p_du_session_data_forwarding_information_response: Option<DataForwardingInformation>,
            n_g_dl_up_unchanged: Option<PDUSessionResourceSetupItemNGDLUPUnchanged>,
            d_rb_setup_list_ng_ran: DRBSetupListNGRAN,
            d_rb_failed_list_ng_ran: Option<DRBFailedListNGRAN>,
            i_e_extensions: Option<PDUSessionResourceSetupItemIEExtensions>,
        ) -> Self {
            Self {
                p_du_session_id,
                security_result,
                n_g_dl_up_tnl_information,
                p_du_session_data_forwarding_information_response,
                n_g_dl_up_unchanged,
                d_rb_setup_list_ng_ran,
                d_rb_failed_list_ng_ran,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "PDU-Session-Resource-Setup-List"
    )]
    pub struct PDUSessionResourceSetupList(pub SequenceOf<PDUSessionResourceSetupItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousPDUSessionResourceSetupModItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPDUSessionResourceSetupModItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousPDUSessionResourceSetupModItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousPDUSessionResourceSetupModItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousPDUSessionResourceSetupModItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct PDUSessionResourceSetupModItemIEExtensions(
        pub SequenceOf<AnonymousPDUSessionResourceSetupModItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "PDU-Session-Resource-Setup-Mod-Item")]
    #[non_exhaustive]
    pub struct PDUSessionResourceSetupModItem {
        #[rasn(identifier = "pDU-Session-ID")]
        pub p_du_session_id: PDUSessionID,
        #[rasn(identifier = "securityResult")]
        pub security_result: Option<SecurityResult>,
        #[rasn(identifier = "nG-DL-UP-TNL-Information")]
        pub n_g_dl_up_tnl_information: UPTNLInformation,
        #[rasn(identifier = "pDU-Session-Data-Forwarding-Information-Response")]
        pub p_du_session_data_forwarding_information_response: Option<DataForwardingInformation>,
        #[rasn(identifier = "dRB-Setup-Mod-List-NG-RAN")]
        pub d_rb_setup_mod_list_ng_ran: DRBSetupModListNGRAN,
        #[rasn(identifier = "dRB-Failed-Mod-List-NG-RAN")]
        pub d_rb_failed_mod_list_ng_ran: Option<DRBFailedModListNGRAN>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<PDUSessionResourceSetupModItemIEExtensions>,
    }
    impl PDUSessionResourceSetupModItem {
        pub fn new(
            p_du_session_id: PDUSessionID,
            security_result: Option<SecurityResult>,
            n_g_dl_up_tnl_information: UPTNLInformation,
            p_du_session_data_forwarding_information_response: Option<DataForwardingInformation>,
            d_rb_setup_mod_list_ng_ran: DRBSetupModListNGRAN,
            d_rb_failed_mod_list_ng_ran: Option<DRBFailedModListNGRAN>,
            i_e_extensions: Option<PDUSessionResourceSetupModItemIEExtensions>,
        ) -> Self {
            Self {
                p_du_session_id,
                security_result,
                n_g_dl_up_tnl_information,
                p_du_session_data_forwarding_information_response,
                d_rb_setup_mod_list_ng_ran,
                d_rb_failed_mod_list_ng_ran,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "PDU-Session-Resource-Setup-Mod-List"
    )]
    pub struct PDUSessionResourceSetupModList(pub SequenceOf<PDUSessionResourceSetupModItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousPDUSessionResourceToModifyItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPDUSessionResourceToModifyItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousPDUSessionResourceToModifyItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousPDUSessionResourceToModifyItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousPDUSessionResourceToModifyItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct PDUSessionResourceToModifyItemIEExtensions(
        pub SequenceOf<AnonymousPDUSessionResourceToModifyItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "PDU-Session-Resource-To-Modify-Item")]
    #[non_exhaustive]
    pub struct PDUSessionResourceToModifyItem {
        #[rasn(identifier = "pDU-Session-ID")]
        pub p_du_session_id: PDUSessionID,
        #[rasn(identifier = "securityIndication")]
        pub security_indication: Option<SecurityIndication>,
        #[rasn(identifier = "pDU-Session-Resource-DL-AMBR")]
        pub p_du_session_resource_dl_ambr: Option<BitRate>,
        #[rasn(identifier = "nG-UL-UP-TNL-Information")]
        pub n_g_ul_up_tnl_information: Option<UPTNLInformation>,
        #[rasn(identifier = "pDU-Session-Data-Forwarding-Information-Request")]
        pub p_du_session_data_forwarding_information_request:
            Option<DataForwardingInformationRequest>,
        #[rasn(identifier = "pDU-Session-Data-Forwarding-Information")]
        pub p_du_session_data_forwarding_information: Option<DataForwardingInformation>,
        #[rasn(identifier = "pDU-Session-Inactivity-Timer")]
        pub p_du_session_inactivity_timer: Option<InactivityTimer>,
        #[rasn(identifier = "networkInstance")]
        pub network_instance: Option<NetworkInstance>,
        #[rasn(identifier = "dRB-To-Setup-List-NG-RAN")]
        pub d_rb_to_setup_list_ng_ran: Option<DRBToSetupListNGRAN>,
        #[rasn(identifier = "dRB-To-Modify-List-NG-RAN")]
        pub d_rb_to_modify_list_ng_ran: Option<DRBToModifyListNGRAN>,
        #[rasn(identifier = "dRB-To-Remove-List-NG-RAN")]
        pub d_rb_to_remove_list_ng_ran: Option<DRBToRemoveListNGRAN>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<PDUSessionResourceToModifyItemIEExtensions>,
    }
    impl PDUSessionResourceToModifyItem {
        pub fn new(
            p_du_session_id: PDUSessionID,
            security_indication: Option<SecurityIndication>,
            p_du_session_resource_dl_ambr: Option<BitRate>,
            n_g_ul_up_tnl_information: Option<UPTNLInformation>,
            p_du_session_data_forwarding_information_request: Option<
                DataForwardingInformationRequest,
            >,
            p_du_session_data_forwarding_information: Option<DataForwardingInformation>,
            p_du_session_inactivity_timer: Option<InactivityTimer>,
            network_instance: Option<NetworkInstance>,
            d_rb_to_setup_list_ng_ran: Option<DRBToSetupListNGRAN>,
            d_rb_to_modify_list_ng_ran: Option<DRBToModifyListNGRAN>,
            d_rb_to_remove_list_ng_ran: Option<DRBToRemoveListNGRAN>,
            i_e_extensions: Option<PDUSessionResourceToModifyItemIEExtensions>,
        ) -> Self {
            Self {
                p_du_session_id,
                security_indication,
                p_du_session_resource_dl_ambr,
                n_g_ul_up_tnl_information,
                p_du_session_data_forwarding_information_request,
                p_du_session_data_forwarding_information,
                p_du_session_inactivity_timer,
                network_instance,
                d_rb_to_setup_list_ng_ran,
                d_rb_to_modify_list_ng_ran,
                d_rb_to_remove_list_ng_ran,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "PDU-Session-Resource-To-Modify-List"
    )]
    pub struct PDUSessionResourceToModifyList(pub SequenceOf<PDUSessionResourceToModifyItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousPDUSessionResourceToRemoveItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPDUSessionResourceToRemoveItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousPDUSessionResourceToRemoveItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousPDUSessionResourceToRemoveItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousPDUSessionResourceToRemoveItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct PDUSessionResourceToRemoveItemIEExtensions(
        pub SequenceOf<AnonymousPDUSessionResourceToRemoveItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "PDU-Session-Resource-To-Remove-Item")]
    #[non_exhaustive]
    pub struct PDUSessionResourceToRemoveItem {
        #[rasn(identifier = "pDU-Session-ID")]
        pub p_du_session_id: PDUSessionID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<PDUSessionResourceToRemoveItemIEExtensions>,
    }
    impl PDUSessionResourceToRemoveItem {
        pub fn new(
            p_du_session_id: PDUSessionID,
            i_e_extensions: Option<PDUSessionResourceToRemoveItemIEExtensions>,
        ) -> Self {
            Self {
                p_du_session_id,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "PDU-Session-Resource-To-Remove-List"
    )]
    pub struct PDUSessionResourceToRemoveList(pub SequenceOf<PDUSessionResourceToRemoveItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousPDUSessionResourceToSetupItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPDUSessionResourceToSetupItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousPDUSessionResourceToSetupItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousPDUSessionResourceToSetupItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousPDUSessionResourceToSetupItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct PDUSessionResourceToSetupItemIEExtensions(
        pub SequenceOf<AnonymousPDUSessionResourceToSetupItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "PDU-Session-Resource-To-Setup-Item")]
    #[non_exhaustive]
    pub struct PDUSessionResourceToSetupItem {
        #[rasn(identifier = "pDU-Session-ID")]
        pub p_du_session_id: PDUSessionID,
        #[rasn(identifier = "pDU-Session-Type")]
        pub p_du_session_type: PDUSessionType,
        #[rasn(identifier = "sNSSAI")]
        pub s_nssai: SNSSAI,
        #[rasn(identifier = "securityIndication")]
        pub security_indication: SecurityIndication,
        #[rasn(identifier = "pDU-Session-Resource-DL-AMBR")]
        pub p_du_session_resource_dl_ambr: Option<BitRate>,
        #[rasn(identifier = "nG-UL-UP-TNL-Information")]
        pub n_g_ul_up_tnl_information: UPTNLInformation,
        #[rasn(identifier = "pDU-Session-Data-Forwarding-Information-Request")]
        pub p_du_session_data_forwarding_information_request:
            Option<DataForwardingInformationRequest>,
        #[rasn(identifier = "pDU-Session-Inactivity-Timer")]
        pub p_du_session_inactivity_timer: Option<InactivityTimer>,
        #[rasn(identifier = "existing-Allocated-NG-DL-UP-TNL-Info")]
        pub existing_allocated_ng_dl_up_tnl_info: Option<UPTNLInformation>,
        #[rasn(identifier = "networkInstance")]
        pub network_instance: Option<NetworkInstance>,
        #[rasn(identifier = "dRB-To-Setup-List-NG-RAN")]
        pub d_rb_to_setup_list_ng_ran: DRBToSetupListNGRAN,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<PDUSessionResourceToSetupItemIEExtensions>,
    }
    impl PDUSessionResourceToSetupItem {
        pub fn new(
            p_du_session_id: PDUSessionID,
            p_du_session_type: PDUSessionType,
            s_nssai: SNSSAI,
            security_indication: SecurityIndication,
            p_du_session_resource_dl_ambr: Option<BitRate>,
            n_g_ul_up_tnl_information: UPTNLInformation,
            p_du_session_data_forwarding_information_request: Option<
                DataForwardingInformationRequest,
            >,
            p_du_session_inactivity_timer: Option<InactivityTimer>,
            existing_allocated_ng_dl_up_tnl_info: Option<UPTNLInformation>,
            network_instance: Option<NetworkInstance>,
            d_rb_to_setup_list_ng_ran: DRBToSetupListNGRAN,
            i_e_extensions: Option<PDUSessionResourceToSetupItemIEExtensions>,
        ) -> Self {
            Self {
                p_du_session_id,
                p_du_session_type,
                s_nssai,
                security_indication,
                p_du_session_resource_dl_ambr,
                n_g_ul_up_tnl_information,
                p_du_session_data_forwarding_information_request,
                p_du_session_inactivity_timer,
                existing_allocated_ng_dl_up_tnl_info,
                network_instance,
                d_rb_to_setup_list_ng_ran,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "PDU-Session-Resource-To-Setup-List"
    )]
    pub struct PDUSessionResourceToSetupList(pub SequenceOf<PDUSessionResourceToSetupItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousPDUSessionResourceToSetupModItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPDUSessionResourceToSetupModItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousPDUSessionResourceToSetupModItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousPDUSessionResourceToSetupModItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousPDUSessionResourceToSetupModItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct PDUSessionResourceToSetupModItemIEExtensions(
        pub SequenceOf<AnonymousPDUSessionResourceToSetupModItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "PDU-Session-Resource-To-Setup-Mod-Item")]
    #[non_exhaustive]
    pub struct PDUSessionResourceToSetupModItem {
        #[rasn(identifier = "pDU-Session-ID")]
        pub p_du_session_id: PDUSessionID,
        #[rasn(identifier = "pDU-Session-Type")]
        pub p_du_session_type: PDUSessionType,
        #[rasn(identifier = "sNSSAI")]
        pub s_nssai: SNSSAI,
        #[rasn(identifier = "securityIndication")]
        pub security_indication: SecurityIndication,
        #[rasn(identifier = "pDU-Session-Resource-AMBR")]
        pub p_du_session_resource_ambr: Option<BitRate>,
        #[rasn(identifier = "nG-UL-UP-TNL-Information")]
        pub n_g_ul_up_tnl_information: UPTNLInformation,
        #[rasn(identifier = "pDU-Session-Data-Forwarding-Information-Request")]
        pub p_du_session_data_forwarding_information_request:
            Option<DataForwardingInformationRequest>,
        #[rasn(identifier = "pDU-Session-Inactivity-Timer")]
        pub p_du_session_inactivity_timer: Option<InactivityTimer>,
        #[rasn(identifier = "dRB-To-Setup-Mod-List-NG-RAN")]
        pub d_rb_to_setup_mod_list_ng_ran: DRBToSetupModListNGRAN,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<PDUSessionResourceToSetupModItemIEExtensions>,
    }
    impl PDUSessionResourceToSetupModItem {
        pub fn new(
            p_du_session_id: PDUSessionID,
            p_du_session_type: PDUSessionType,
            s_nssai: SNSSAI,
            security_indication: SecurityIndication,
            p_du_session_resource_ambr: Option<BitRate>,
            n_g_ul_up_tnl_information: UPTNLInformation,
            p_du_session_data_forwarding_information_request: Option<
                DataForwardingInformationRequest,
            >,
            p_du_session_inactivity_timer: Option<InactivityTimer>,
            d_rb_to_setup_mod_list_ng_ran: DRBToSetupModListNGRAN,
            i_e_extensions: Option<PDUSessionResourceToSetupModItemIEExtensions>,
        ) -> Self {
            Self {
                p_du_session_id,
                p_du_session_type,
                s_nssai,
                security_indication,
                p_du_session_resource_ambr,
                n_g_ul_up_tnl_information,
                p_du_session_data_forwarding_information_request,
                p_du_session_inactivity_timer,
                d_rb_to_setup_mod_list_ng_ran,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "PDU-Session-Resource-To-Setup-Mod-List"
    )]
    pub struct PDUSessionResourceToSetupModList(pub SequenceOf<PDUSessionResourceToSetupModItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousPDUSessionToNotifyItemIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPDUSessionToNotifyItemIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousPDUSessionToNotifyItemIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousPDUSessionToNotifyItemIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousPDUSessionToNotifyItemIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct PDUSessionToNotifyItemIEExtensions(
        pub SequenceOf<AnonymousPDUSessionToNotifyItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "PDU-Session-To-Notify-Item")]
    #[non_exhaustive]
    pub struct PDUSessionToNotifyItem {
        #[rasn(identifier = "pDU-Session-ID")]
        pub p_du_session_id: PDUSessionID,
        #[rasn(identifier = "qoS-Flow-List")]
        pub qo_s_flow_list: QoSFlowList,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<PDUSessionToNotifyItemIEExtensions>,
    }
    impl PDUSessionToNotifyItem {
        pub fn new(
            p_du_session_id: PDUSessionID,
            qo_s_flow_list: QoSFlowList,
            i_e_extensions: Option<PDUSessionToNotifyItemIEExtensions>,
        ) -> Self {
            Self {
                p_du_session_id,
                qo_s_flow_list,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"), identifier = "PDU-Session-To-Notify-List")]
    pub struct PDUSessionToNotifyList(pub SequenceOf<PDUSessionToNotifyItem>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "PDU-Session-Type")]
    #[non_exhaustive]
    pub enum PDUSessionType {
        ipv4 = 0,
        ipv6 = 1,
        ipv4v6 = 2,
        ethernet = 3,
        unstructured = 4,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "PDUSession-PairID",
        value("0..=255", extensible)
    )]
    pub struct PDUSessionPairID(pub Integer);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PDUSetQoSInformationPduSetIntegratedHandlingInformation {
        #[rasn(identifier = "true")]
        R_true = 0,
        #[rasn(identifier = "false")]
        R_false = 1,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousPDUSetQoSInformationIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPDUSetQoSInformationIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousPDUSetQoSInformationIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousPDUSetQoSInformationIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousPDUSetQoSInformationIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct PDUSetQoSInformationIEExtensions(
        pub SequenceOf<AnonymousPDUSetQoSInformationIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct PDUSetQoSInformation {
        #[rasn(identifier = "pduSetDelayBudget")]
        pub pdu_set_delay_budget: Option<ExtendedPacketDelayBudget>,
        #[rasn(identifier = "pduSetErrorRate")]
        pub pdu_set_error_rate: Option<PacketErrorRate>,
        #[rasn(identifier = "pduSetIntegratedHandlingInformation")]
        pub pdu_set_integrated_handling_information:
            Option<PDUSetQoSInformationPduSetIntegratedHandlingInformation>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<PDUSetQoSInformationIEExtensions>,
    }
    impl PDUSetQoSInformation {
        pub fn new(
            pdu_set_delay_budget: Option<ExtendedPacketDelayBudget>,
            pdu_set_error_rate: Option<PacketErrorRate>,
            pdu_set_integrated_handling_information: Option<
                PDUSetQoSInformationPduSetIntegratedHandlingInformation,
            >,
            i_e_extensions: Option<PDUSetQoSInformationIEExtensions>,
        ) -> Self {
            Self {
                pdu_set_delay_budget,
                pdu_set_error_rate,
                pdu_set_integrated_handling_information,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousPDUSetQoSParametersIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPDUSetQoSParametersIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousPDUSetQoSParametersIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousPDUSetQoSParametersIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousPDUSetQoSParametersIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct PDUSetQoSParametersIEExtensions(
        pub SequenceOf<AnonymousPDUSetQoSParametersIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct PDUSetQoSParameters {
        #[rasn(identifier = "ulPDUSetQoSInformation")]
        pub ul_pduset_qo_sinformation: Option<PDUSetQoSInformation>,
        #[rasn(identifier = "dlPDUSetQoSInformation")]
        pub dl_pduset_qo_sinformation: Option<PDUSetQoSInformation>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<PDUSetQoSParametersIEExtensions>,
    }
    impl PDUSetQoSParameters {
        pub fn new(
            ul_pduset_qo_sinformation: Option<PDUSetQoSInformation>,
            dl_pduset_qo_sinformation: Option<PDUSetQoSInformation>,
            i_e_extensions: Option<PDUSetQoSParametersIEExtensions>,
        ) -> Self {
            Self {
                ul_pduset_qo_sinformation,
                dl_pduset_qo_sinformation,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PDUSetbasedHandlingIndicator {
        supported = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "PER-Exponent", value("0..=9", extensible))]
    pub struct PERExponent(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "PER-Scalar", value("0..=9", extensible))]
    pub struct PERScalar(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "PLMN-Identity")]
    pub struct PLMNIdentity(pub FixedOctetString<3usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=7", extensible))]
    pub struct PPI(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PSIbasedDiscardTimer {
        ms0 = 0,
        ms2 = 1,
        ms4 = 2,
        ms6 = 3,
        ms8 = 4,
        ms10 = 5,
        ms12 = 6,
        ms14 = 7,
        ms18 = 8,
        ms22 = 9,
        ms26 = 10,
        ms30 = 11,
        ms40 = 12,
        ms50 = 13,
        ms75 = 14,
        ms100 = 15,
    }
    #[doc = " P"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=1023", extensible))]
    pub struct PacketDelayBudget(pub Integer);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousPacketErrorRateIEExtensionsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPacketErrorRateIEExtensions {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousPacketErrorRateIEExtensionsCriticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousPacketErrorRateIEExtensions {
        pub fn new(
            id: u16,
            criticality: AnonymousPacketErrorRateIEExtensionsCriticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct PacketErrorRateIEExtensions(pub SequenceOf<AnonymousPacketErrorRateIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PacketErrorRate {
        #[rasn(identifier = "pER-Scalar")]
        pub p_er_scalar: PERScalar,
        #[rasn(identifier = "pER-Exponent")]
        pub p_er_exponent: PERExponent,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<PacketErrorRateIEExtensions>,
    }
    impl PacketErrorRate {
        pub fn new(
            p_er_scalar: PERScalar,
            p_er_exponent: PERExponent,
            i_e_extensions: Option<PacketErrorRateIEExtensions>,
        ) -> Self {
            Self {
                p_er_scalar,
                p_er_exponent,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=640000", extensible))]
    pub struct Periodicity(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct PortNumber(pub FixedBitString<16usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "Pre-emptionCapability")]
    pub enum PreEmptionCapability {
        #[rasn(identifier = "shall-not-trigger-pre-emption")]
        shall_not_trigger_pre_emption = 0,
        #[rasn(identifier = "may-trigger-pre-emption")]
        may_trigger_pre_emption = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "Pre-emptionVulnerability")]
    pub enum PreEmptionVulnerability {
        #[rasn(identifier = "not-pre-emptable")]
        not_pre_emptable = 0,
        #[rasn(identifier = "pre-emptable")]
        pre_emptable = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct PriorityLevel(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PrivacyIndicator {
        #[rasn(identifier = "immediate-MDT")]
        immediate_MDT = 0,
        #[rasn(identifier = "logged-MDT")]
        logged_MDT = 1,
    }
    #[doc = " Q"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct QCI(pub u8);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct QoSCharacteristicsChoiceExtension {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl QoSCharacteristicsChoiceExtension {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags, identifier = "QoS-Characteristics")]
    pub enum QoSCharacteristics {
        #[rasn(identifier = "non-Dynamic-5QI")]
        non_Dynamic_5QI(NonDynamic5QIDescriptor),
        #[rasn(identifier = "dynamic-5QI")]
        dynamic_5QI(Dynamic5QIDescriptor),
        #[rasn(identifier = "choice-extension")]
        choice_extension(QoSCharacteristicsChoiceExtension),
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousQoSFlowFailedItemIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousQoSFlowFailedItemIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct QoSFlowFailedItemIEExtensions(
        pub SequenceOf<AnonymousQoSFlowFailedItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "QoS-Flow-Failed-Item")]
    #[non_exhaustive]
    pub struct QoSFlowFailedItem {
        #[rasn(identifier = "qoS-Flow-Identifier")]
        pub qo_s_flow_identifier: QoSFlowIdentifier,
        pub cause: Cause,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<QoSFlowFailedItemIEExtensions>,
    }
    impl QoSFlowFailedItem {
        pub fn new(
            qo_s_flow_identifier: QoSFlowIdentifier,
            cause: Cause,
            i_e_extensions: Option<QoSFlowFailedItemIEExtensions>,
        ) -> Self {
            Self {
                qo_s_flow_identifier,
                cause,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=64"), identifier = "QoS-Flow-Failed-List")]
    pub struct QoSFlowFailedList(pub SequenceOf<QoSFlowFailedItem>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "QoS-Flow-Identifier", value("0..=63"))]
    pub struct QoSFlowIdentifier(pub u8);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousQoSFlowItemIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousQoSFlowItemIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct QoSFlowItemIEExtensions(pub SequenceOf<AnonymousQoSFlowItemIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "QoS-Flow-Item")]
    #[non_exhaustive]
    pub struct QoSFlowItem {
        #[rasn(identifier = "qoS-Flow-Identifier")]
        pub qo_s_flow_identifier: QoSFlowIdentifier,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<QoSFlowItemIEExtensions>,
    }
    impl QoSFlowItem {
        pub fn new(
            qo_s_flow_identifier: QoSFlowIdentifier,
            i_e_extensions: Option<QoSFlowItemIEExtensions>,
        ) -> Self {
            Self {
                qo_s_flow_identifier,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=64"), identifier = "QoS-Flow-List")]
    pub struct QoSFlowList(pub SequenceOf<QoSFlowItem>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "QoS-Flow-Mapping-Indication")]
    #[non_exhaustive]
    pub enum QoSFlowMappingIndication {
        ul = 0,
        dl = 1,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousQoSFlowMappingItemIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousQoSFlowMappingItemIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct QoSFlowMappingItemIEExtensions(
        pub SequenceOf<AnonymousQoSFlowMappingItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "QoS-Flow-Mapping-Item")]
    #[non_exhaustive]
    pub struct QoSFlowMappingItem {
        #[rasn(identifier = "qoS-Flow-Identifier")]
        pub qo_s_flow_identifier: QoSFlowIdentifier,
        #[rasn(identifier = "qoSFlowMappingIndication")]
        pub qo_sflow_mapping_indication: Option<QoSFlowMappingIndication>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<QoSFlowMappingItemIEExtensions>,
    }
    impl QoSFlowMappingItem {
        pub fn new(
            qo_s_flow_identifier: QoSFlowIdentifier,
            qo_sflow_mapping_indication: Option<QoSFlowMappingIndication>,
            i_e_extensions: Option<QoSFlowMappingItemIEExtensions>,
        ) -> Self {
            Self {
                qo_s_flow_identifier,
                qo_sflow_mapping_indication,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=64"), identifier = "QoS-Flow-Mapping-List")]
    pub struct QoSFlowMappingList(pub SequenceOf<QoSFlowMappingItem>);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousQoSFlowQoSParameterItemIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousQoSFlowQoSParameterItemIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct QoSFlowQoSParameterItemIEExtensions(
        pub SequenceOf<AnonymousQoSFlowQoSParameterItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "QoS-Flow-QoS-Parameter-Item")]
    #[non_exhaustive]
    pub struct QoSFlowQoSParameterItem {
        #[rasn(identifier = "qoS-Flow-Identifier")]
        pub qo_s_flow_identifier: QoSFlowIdentifier,
        #[rasn(identifier = "qoSFlowLevelQoSParameters")]
        pub qo_sflow_level_qo_sparameters: QoSFlowLevelQoSParameters,
        #[rasn(identifier = "qoSFlowMappingIndication")]
        pub qo_sflow_mapping_indication: Option<QoSFlowMappingIndication>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<QoSFlowQoSParameterItemIEExtensions>,
    }
    impl QoSFlowQoSParameterItem {
        pub fn new(
            qo_s_flow_identifier: QoSFlowIdentifier,
            qo_sflow_level_qo_sparameters: QoSFlowLevelQoSParameters,
            qo_sflow_mapping_indication: Option<QoSFlowMappingIndication>,
            i_e_extensions: Option<QoSFlowQoSParameterItemIEExtensions>,
        ) -> Self {
            Self {
                qo_s_flow_identifier,
                qo_sflow_level_qo_sparameters,
                qo_sflow_mapping_indication,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=64"), identifier = "QoS-Flow-QoS-Parameter-List")]
    pub struct QoSFlowQoSParameterList(pub SequenceOf<QoSFlowQoSParameterItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum QoSFlowRemovedItemQoSFlowReleasedInSession {
        #[rasn(identifier = "released-in-session")]
        released_in_session = 0,
        #[rasn(identifier = "not-released-in-session")]
        not_released_in_session = 1,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousQoSFlowRemovedItemIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousQoSFlowRemovedItemIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct QoSFlowRemovedItemIEExtensions(
        pub SequenceOf<AnonymousQoSFlowRemovedItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "QoS-Flow-Removed-Item")]
    #[non_exhaustive]
    pub struct QoSFlowRemovedItem {
        #[rasn(identifier = "qoS-Flow-Identifier")]
        pub qo_s_flow_identifier: QoSFlowIdentifier,
        #[rasn(identifier = "qoS-Flow-Released-In-Session")]
        pub qo_s_flow_released_in_session: Option<QoSFlowRemovedItemQoSFlowReleasedInSession>,
        #[rasn(size("5"), identifier = "qoS-Flow-Accumulated-Session-Time")]
        pub qo_s_flow_accumulated_session_time: Option<OctetString>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<QoSFlowRemovedItemIEExtensions>,
    }
    impl QoSFlowRemovedItem {
        pub fn new(
            qo_s_flow_identifier: QoSFlowIdentifier,
            qo_s_flow_released_in_session: Option<QoSFlowRemovedItemQoSFlowReleasedInSession>,
            qo_s_flow_accumulated_session_time: Option<OctetString>,
            i_e_extensions: Option<QoSFlowRemovedItemIEExtensions>,
        ) -> Self {
            Self {
                qo_s_flow_identifier,
                qo_s_flow_released_in_session,
                qo_s_flow_accumulated_session_time,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "QoS-Flows-DRB-Remapping")]
    #[non_exhaustive]
    pub enum QoSFlowsDRBRemapping {
        update = 0,
        #[rasn(identifier = "source-configuration")]
        source_configuration = 1,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousQoSFlowsToBeForwardedItemIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousQoSFlowsToBeForwardedItemIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct QoSFlowsToBeForwardedItemIEExtensions(
        pub SequenceOf<AnonymousQoSFlowsToBeForwardedItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "QoS-Flows-to-be-forwarded-Item")]
    #[non_exhaustive]
    pub struct QoSFlowsToBeForwardedItem {
        #[rasn(identifier = "qoS-Flow-Identifier")]
        pub qo_s_flow_identifier: QoSFlowIdentifier,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<QoSFlowsToBeForwardedItemIEExtensions>,
    }
    impl QoSFlowsToBeForwardedItem {
        pub fn new(
            qo_s_flow_identifier: QoSFlowIdentifier,
            i_e_extensions: Option<QoSFlowsToBeForwardedItemIEExtensions>,
        ) -> Self {
            Self {
                qo_s_flow_identifier,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=64"),
        identifier = "QoS-Flows-to-be-forwarded-List"
    )]
    pub struct QoSFlowsToBeForwardedList(pub SequenceOf<QoSFlowsToBeForwardedItem>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "QoS-Mapping-Information")]
    #[non_exhaustive]
    pub struct QoSMappingInformation {
        #[rasn(size("6"))]
        pub dscp: Option<BitString>,
        #[rasn(size("20"), identifier = "flow-label")]
        pub flow_label: Option<BitString>,
    }
    impl QoSMappingInformation {
        pub fn new(dscp: Option<BitString>, flow_label: Option<BitString>) -> Self {
            Self { dscp, flow_label }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousQoSParametersSupportListIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousQoSParametersSupportListIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct QoSParametersSupportListIEExtensions(
        pub SequenceOf<AnonymousQoSParametersSupportListIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "QoS-Parameters-Support-List")]
    #[non_exhaustive]
    pub struct QoSParametersSupportList {
        #[rasn(identifier = "eUTRAN-QoS-Support-List")]
        pub e_utran_qo_s_support_list: Option<EUTRANQoSSupportList>,
        #[rasn(identifier = "nG-RAN-QoS-Support-List")]
        pub n_g_ran_qo_s_support_list: Option<NGRANQoSSupportList>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<QoSParametersSupportListIEExtensions>,
    }
    impl QoSParametersSupportList {
        pub fn new(
            e_utran_qo_s_support_list: Option<EUTRANQoSSupportList>,
            n_g_ran_qo_s_support_list: Option<NGRANQoSSupportList>,
            i_e_extensions: Option<QoSParametersSupportListIEExtensions>,
        ) -> Self {
            Self {
                e_utran_qo_s_support_list,
                n_g_ran_qo_s_support_list,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum QoSFlowLevelQoSParametersReflectiveQoSAttribute {
        #[rasn(identifier = "subject-to")]
        subject_to = 0,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum QoSFlowLevelQoSParametersAdditionalQoSInformation {
        #[rasn(identifier = "more-likely")]
        more_likely = 0,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum QoSFlowLevelQoSParametersReflectiveQoSIndicator {
        enabled = 0,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousQoSFlowLevelQoSParametersIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousQoSFlowLevelQoSParametersIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct QoSFlowLevelQoSParametersIEExtensions(
        pub SequenceOf<AnonymousQoSFlowLevelQoSParametersIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct QoSFlowLevelQoSParameters {
        #[rasn(identifier = "qoS-Characteristics")]
        pub qo_s_characteristics: QoSCharacteristics,
        #[rasn(identifier = "nGRANallocationRetentionPriority")]
        pub n_granallocation_retention_priority: NGRANAllocationAndRetentionPriority,
        #[rasn(identifier = "gBR-QoS-Flow-Information")]
        pub g_br_qo_s_flow_information: Option<GBRQoSFlowInformation>,
        #[rasn(identifier = "reflective-QoS-Attribute")]
        pub reflective_qo_s_attribute: Option<QoSFlowLevelQoSParametersReflectiveQoSAttribute>,
        #[rasn(identifier = "additional-QoS-Information")]
        pub additional_qo_s_information: Option<QoSFlowLevelQoSParametersAdditionalQoSInformation>,
        #[rasn(value("1..=8", extensible), identifier = "paging-Policy-Index")]
        pub paging_policy_index: Option<Integer>,
        #[rasn(identifier = "reflective-QoS-Indicator")]
        pub reflective_qo_s_indicator: Option<QoSFlowLevelQoSParametersReflectiveQoSIndicator>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<QoSFlowLevelQoSParametersIEExtensions>,
    }
    impl QoSFlowLevelQoSParameters {
        pub fn new(
            qo_s_characteristics: QoSCharacteristics,
            n_granallocation_retention_priority: NGRANAllocationAndRetentionPriority,
            g_br_qo_s_flow_information: Option<GBRQoSFlowInformation>,
            reflective_qo_s_attribute: Option<QoSFlowLevelQoSParametersReflectiveQoSAttribute>,
            additional_qo_s_information: Option<QoSFlowLevelQoSParametersAdditionalQoSInformation>,
            paging_policy_index: Option<Integer>,
            reflective_qo_s_indicator: Option<QoSFlowLevelQoSParametersReflectiveQoSIndicator>,
            i_e_extensions: Option<QoSFlowLevelQoSParametersIEExtensions>,
        ) -> Self {
            Self {
                qo_s_characteristics,
                n_granallocation_retention_priority,
                g_br_qo_s_flow_information,
                reflective_qo_s_attribute,
                additional_qo_s_information,
                paging_policy_index,
                reflective_qo_s_indicator,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=127", extensible))]
    pub struct QoSPriorityLevel(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum QosMonitoringDisabled {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=1800", extensible))]
    pub struct QosMonitoringReportingFrequency(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum QosMonitoringRequest {
        ul = 0,
        dl = 1,
        both = 2,
    }
    #[doc = " R"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct RANUEID(pub FixedOctetString<8usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "RAT-Type")]
    #[non_exhaustive]
    pub enum RATType {
        #[rasn(identifier = "e-UTRA")]
        e_UTRA = 0,
        nR = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "RLC-Mode")]
    #[non_exhaustive]
    pub enum RLCMode {
        #[rasn(identifier = "rlc-tm")]
        rlc_tm = 0,
        #[rasn(identifier = "rlc-am")]
        rlc_am = 1,
        #[rasn(identifier = "rlc-um-bidirectional")]
        rlc_um_bidirectional = 2,
        #[rasn(identifier = "rlc-um-unidirectional-ul")]
        rlc_um_unidirectional_ul = 3,
        #[rasn(identifier = "rlc-um-unidirectional-dl")]
        rlc_um_unidirectional_dl = 4,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ROHCContinueROHC {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousROHCIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousROHCIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct ROHCIEExtensions(pub SequenceOf<AnonymousROHCIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ROHC {
        #[rasn(value("0..=16383", extensible), identifier = "maxCID")]
        pub max_cid: Integer,
        #[rasn(value("0..=511", extensible), identifier = "rOHC-Profiles")]
        pub r_ohc_profiles: Integer,
        #[rasn(identifier = "continueROHC")]
        pub continue_rohc: Option<ROHCContinueROHC>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ROHCIEExtensions>,
    }
    impl ROHC {
        pub fn new(
            max_cid: Integer,
            r_ohc_profiles: Integer,
            continue_rohc: Option<ROHCContinueROHC>,
            i_e_extensions: Option<ROHCIEExtensions>,
        ) -> Self {
            Self {
                max_cid,
                r_ohc_profiles,
                continue_rohc,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ROHCParametersChoiceExtension {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl ROHCParametersChoiceExtension {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags, identifier = "ROHC-Parameters")]
    pub enum ROHCParameters {
        rOHC(ROHC),
        uPlinkOnlyROHC(UplinkOnlyROHC),
        #[rasn(identifier = "choice-Extension")]
        choice_Extension(ROHCParametersChoiceExtension),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum RSN {
        v1 = 0,
        v2 = 1,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousRedundantPDUSessionInformationIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousRedundantPDUSessionInformationIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct RedundantPDUSessionInformationIEExtensions(
        pub SequenceOf<AnonymousRedundantPDUSessionInformationIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RedundantPDUSessionInformation {
        #[rasn(identifier = "rSN")]
        pub r_sn: RSN,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<RedundantPDUSessionInformationIEExtensions>,
    }
    impl RedundantPDUSessionInformation {
        pub fn new(
            r_sn: RSN,
            i_e_extensions: Option<RedundantPDUSessionInformationIEExtensions>,
        ) -> Self {
            Self {
                r_sn,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum RedundantQoSFlowIndicator {
        #[rasn(identifier = "true")]
        R_true = 0,
        #[rasn(identifier = "false")]
        R_false = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum RegistrationRequest {
        start = 0,
        stop = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct ReportCharacteristics(pub FixedBitString<36usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ReportingPeriodicity {
        ms500 = 0,
        ms1000 = 1,
        ms2000 = 2,
        ms5000 = 3,
        ms10000 = 4,
        ms20000 = 5,
        ms30000 = 6,
        ms40000 = 7,
        ms50000 = 8,
        ms60000 = 9,
        ms70000 = 10,
        ms80000 = 11,
        ms90000 = 12,
        ms100000 = 13,
        ms110000 = 14,
        ms120000 = 15,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum RequestedAction4AvailNGUTermination {
        #[rasn(identifier = "apply-available-configuration")]
        apply_available_configuration = 0,
        #[rasn(identifier = "apply-requested-configuration")]
        apply_requested_configuration = 1,
        #[rasn(
            extension_addition,
            identifier = "apply-available-configuration-if-same-as-requested"
        )]
        apply_available_configuration_if_same_as_requested = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct RetainabilityMeasurementsInfo(pub SequenceOf<DRBRemovedItem>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum SCGActivationStatus {
        #[rasn(identifier = "scg-activated")]
        scg_activated = 0,
        #[rasn(identifier = "scg-deactivated")]
        scg_deactivated = 1,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSDAPConfigurationIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousSDAPConfigurationIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct SDAPConfigurationIEExtensions(
        pub SequenceOf<AnonymousSDAPConfigurationIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SDAP-Configuration")]
    #[non_exhaustive]
    pub struct SDAPConfiguration {
        #[rasn(identifier = "defaultDRB")]
        pub default_drb: DefaultDRB,
        #[rasn(identifier = "sDAP-Header-UL")]
        pub s_dap_header_ul: SDAPHeaderUL,
        #[rasn(identifier = "sDAP-Header-DL")]
        pub s_dap_header_dl: SDAPHeaderDL,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<SDAPConfigurationIEExtensions>,
    }
    impl SDAPConfiguration {
        pub fn new(
            default_drb: DefaultDRB,
            s_dap_header_ul: SDAPHeaderUL,
            s_dap_header_dl: SDAPHeaderDL,
            i_e_extensions: Option<SDAPConfigurationIEExtensions>,
        ) -> Self {
            Self {
                default_drb,
                s_dap_header_ul,
                s_dap_header_dl,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "SDAP-Header-DL")]
    #[non_exhaustive]
    pub enum SDAPHeaderDL {
        present = 0,
        absent = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "SDAP-Header-UL")]
    #[non_exhaustive]
    pub enum SDAPHeaderUL {
        present = 0,
        absent = 1,
    }
    #[doc = " S"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "SDT-data-size-threshold",
        value("1..=192000", extensible)
    )]
    pub struct SDTDataSizeThreshold(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "SDT-data-size-threshold-Crossed")]
    #[non_exhaustive]
    pub enum SDTDataSizeThresholdCrossed {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum SDTContinueROHC {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum SDTindicatorMod {
        #[rasn(identifier = "true")]
        R_true = 0,
        #[rasn(identifier = "false")]
        R_false = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum SDTindicatorSetup {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSNSSAIIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousSNSSAIIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct SNSSAIIEExtensions(pub SequenceOf<AnonymousSNSSAIIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SNSSAI {
        #[rasn(size("1"), identifier = "sST")]
        pub s_st: OctetString,
        #[rasn(size("3"), identifier = "sD")]
        pub s_d: Option<OctetString>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<SNSSAIIEExtensions>,
    }
    impl SNSSAI {
        pub fn new(
            s_st: OctetString,
            s_d: Option<OctetString>,
            i_e_extensions: Option<SNSSAIIEExtensions>,
        ) -> Self {
            Self {
                s_st,
                s_d,
                i_e_extensions,
            }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSecurityAlgorithmIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousSecurityAlgorithmIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct SecurityAlgorithmIEExtensions(
        pub SequenceOf<AnonymousSecurityAlgorithmIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SecurityAlgorithm {
        #[rasn(identifier = "cipheringAlgorithm")]
        pub ciphering_algorithm: CipheringAlgorithm,
        #[rasn(identifier = "integrityProtectionAlgorithm")]
        pub integrity_protection_algorithm: Option<IntegrityProtectionAlgorithm>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<SecurityAlgorithmIEExtensions>,
    }
    impl SecurityAlgorithm {
        pub fn new(
            ciphering_algorithm: CipheringAlgorithm,
            integrity_protection_algorithm: Option<IntegrityProtectionAlgorithm>,
            i_e_extensions: Option<SecurityAlgorithmIEExtensions>,
        ) -> Self {
            Self {
                ciphering_algorithm,
                integrity_protection_algorithm,
                i_e_extensions,
            }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSecurityIndicationIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousSecurityIndicationIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct SecurityIndicationIEExtensions(
        pub SequenceOf<AnonymousSecurityIndicationIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SecurityIndication {
        #[rasn(identifier = "integrityProtectionIndication")]
        pub integrity_protection_indication: IntegrityProtectionIndication,
        #[rasn(identifier = "confidentialityProtectionIndication")]
        pub confidentiality_protection_indication: ConfidentialityProtectionIndication,
        #[rasn(identifier = "maximumIPdatarate")]
        pub maximum_ipdatarate: Option<MaximumIPdatarate>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<SecurityIndicationIEExtensions>,
    }
    impl SecurityIndication {
        pub fn new(
            integrity_protection_indication: IntegrityProtectionIndication,
            confidentiality_protection_indication: ConfidentialityProtectionIndication,
            maximum_ipdatarate: Option<MaximumIPdatarate>,
            i_e_extensions: Option<SecurityIndicationIEExtensions>,
        ) -> Self {
            Self {
                integrity_protection_indication,
                confidentiality_protection_indication,
                maximum_ipdatarate,
                i_e_extensions,
            }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSecurityInformationIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousSecurityInformationIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct SecurityInformationIEExtensions(
        pub SequenceOf<AnonymousSecurityInformationIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SecurityInformation {
        #[rasn(identifier = "securityAlgorithm")]
        pub security_algorithm: SecurityAlgorithm,
        #[rasn(identifier = "uPSecuritykey")]
        pub u_psecuritykey: UPSecuritykey,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<SecurityInformationIEExtensions>,
    }
    impl SecurityInformation {
        pub fn new(
            security_algorithm: SecurityAlgorithm,
            u_psecuritykey: UPSecuritykey,
            i_e_extensions: Option<SecurityInformationIEExtensions>,
        ) -> Self {
            Self {
                security_algorithm,
                u_psecuritykey,
                i_e_extensions,
            }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSecurityResultIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousSecurityResultIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct SecurityResultIEExtensions(pub SequenceOf<AnonymousSecurityResultIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SecurityResult {
        #[rasn(identifier = "integrityProtectionResult")]
        pub integrity_protection_result: IntegrityProtectionResult,
        #[rasn(identifier = "confidentialityProtectionResult")]
        pub confidentiality_protection_result: ConfidentialityProtectionResult,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<SecurityResultIEExtensions>,
    }
    impl SecurityResult {
        pub fn new(
            integrity_protection_result: IntegrityProtectionResult,
            confidentiality_protection_result: ConfidentialityProtectionResult,
            i_e_extensions: Option<SecurityResultIEExtensions>,
        ) -> Self {
            Self {
                integrity_protection_result,
                confidentiality_protection_result,
                i_e_extensions,
            }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSliceSupportItemIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousSliceSupportItemIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct SliceSupportItemIEExtensions(pub SequenceOf<AnonymousSliceSupportItemIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Slice-Support-Item")]
    pub struct SliceSupportItem {
        #[rasn(identifier = "sNSSAI")]
        pub s_nssai: SNSSAI,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<SliceSupportItemIEExtensions>,
    }
    impl SliceSupportItem {
        pub fn new(s_nssai: SNSSAI, i_e_extensions: Option<SliceSupportItemIEExtensions>) -> Self {
            Self {
                s_nssai,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=1024"), identifier = "Slice-Support-List")]
    pub struct SliceSupportList(pub SequenceOf<SliceSupportItem>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum SpecialTriggeringPurpose {
        #[rasn(identifier = "indirect-data-forwarding")]
        indirect_data_forwarding = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=256", extensible))]
    pub struct SubscriberProfileIDforRFP(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=1920000", extensible))]
    pub struct SurvivalTime(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "T-Reordering")]
    #[non_exhaustive]
    pub enum TReordering {
        ms0 = 0,
        ms1 = 1,
        ms2 = 2,
        ms4 = 3,
        ms5 = 4,
        ms8 = 5,
        ms10 = 6,
        ms15 = 7,
        ms20 = 8,
        ms30 = 9,
        ms40 = 10,
        ms50 = 11,
        ms60 = 12,
        ms80 = 13,
        ms100 = 14,
        ms120 = 15,
        ms140 = 16,
        ms160 = 17,
        ms180 = 18,
        ms200 = 19,
        ms220 = 20,
        ms240 = 21,
        ms260 = 22,
        ms280 = 23,
        ms300 = 24,
        ms500 = 25,
        ms750 = 26,
        ms1000 = 27,
        ms1250 = 28,
        ms1500 = 29,
        ms1750 = 30,
        ms2000 = 31,
        ms2250 = 32,
        ms2500 = 33,
        ms2750 = 34,
        ms3000 = 35,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousTReorderingTimerIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousTReorderingTimerIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct TReorderingTimerIEExtensions(pub SequenceOf<AnonymousTReorderingTimerIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "T-ReorderingTimer")]
    #[non_exhaustive]
    pub struct TReorderingTimer {
        #[rasn(identifier = "t-Reordering")]
        pub t_reordering: TReordering,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<TReorderingTimerIEExtensions>,
    }
    impl TReorderingTimer {
        pub fn new(
            t_reordering: TReordering,
            i_e_extensions: Option<TReorderingTimerIEExtensions>,
        ) -> Self {
            Self {
                t_reordering,
                i_e_extensions,
            }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousTNLAvailableCapacityIndicatorIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousTNLAvailableCapacityIndicatorIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct TNLAvailableCapacityIndicatorIEExtensions(
        pub SequenceOf<AnonymousTNLAvailableCapacityIndicatorIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "TNL-AvailableCapacityIndicator")]
    #[non_exhaustive]
    pub struct TNLAvailableCapacityIndicator {
        #[rasn(
            value("0..=16777216", extensible),
            identifier = "dL-TNL-OfferedCapacity"
        )]
        pub d_l_tnl_offered_capacity: Integer,
        #[rasn(value("0..=100", extensible), identifier = "dL-TNL-AvailableCapacity")]
        pub d_l_tnl_available_capacity: Integer,
        #[rasn(
            value("0..=16777216", extensible),
            identifier = "uL-TNL-OfferedCapacity"
        )]
        pub u_l_tnl_offered_capacity: Integer,
        #[rasn(value("0..=100", extensible), identifier = "uL-TNL-AvailableCapacity")]
        pub u_l_tnl_available_capacity: Integer,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<TNLAvailableCapacityIndicatorIEExtensions>,
    }
    impl TNLAvailableCapacityIndicator {
        pub fn new(
            d_l_tnl_offered_capacity: Integer,
            d_l_tnl_available_capacity: Integer,
            u_l_tnl_offered_capacity: Integer,
            u_l_tnl_available_capacity: Integer,
            i_e_extensions: Option<TNLAvailableCapacityIndicatorIEExtensions>,
        ) -> Self {
            Self {
                d_l_tnl_offered_capacity,
                d_l_tnl_available_capacity,
                u_l_tnl_offered_capacity,
                u_l_tnl_available_capacity,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum TNLAssociationUsage {
        ue = 0,
        #[rasn(identifier = "non-ue")]
        non_ue = 1,
        both = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousTSCAssistanceInformationIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousTSCAssistanceInformationIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct TSCAssistanceInformationIEExtensions(
        pub SequenceOf<AnonymousTSCAssistanceInformationIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct TSCAssistanceInformation {
        pub periodicity: Periodicity,
        #[rasn(identifier = "burstArrivalTime")]
        pub burst_arrival_time: Option<BurstArrivalTime>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<TSCAssistanceInformationIEExtensions>,
    }
    impl TSCAssistanceInformation {
        pub fn new(
            periodicity: Periodicity,
            burst_arrival_time: Option<BurstArrivalTime>,
            i_e_extensions: Option<TSCAssistanceInformationIEExtensions>,
        ) -> Self {
            Self {
                periodicity,
                burst_arrival_time,
                i_e_extensions,
            }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousTSCTrafficCharacteristicsIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousTSCTrafficCharacteristicsIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct TSCTrafficCharacteristicsIEExtensions(
        pub SequenceOf<AnonymousTSCTrafficCharacteristicsIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct TSCTrafficCharacteristics {
        #[rasn(identifier = "tSCTrafficCharacteristicsUL")]
        pub t_sctraffic_characteristics_ul: Option<TSCAssistanceInformation>,
        #[rasn(identifier = "tSCTrafficCharacteristicsDL")]
        pub t_sctraffic_characteristics_dl: Option<TSCAssistanceInformation>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<TSCTrafficCharacteristicsIEExtensions>,
    }
    impl TSCTrafficCharacteristics {
        pub fn new(
            t_sctraffic_characteristics_ul: Option<TSCAssistanceInformation>,
            t_sctraffic_characteristics_dl: Option<TSCAssistanceInformation>,
            i_e_extensions: Option<TSCTrafficCharacteristicsIEExtensions>,
        ) -> Self {
            Self {
                t_sctraffic_characteristics_ul,
                t_sctraffic_characteristics_dl,
                i_e_extensions,
            }
        }
    }
    #[doc = " T"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum TimeToWait {
        v1s = 0,
        v2s = 1,
        v5s = 2,
        v10s = 3,
        v20s = 4,
        v60s = 5,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousTraceActivationIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousTraceActivationIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct TraceActivationIEExtensions(pub SequenceOf<AnonymousTraceActivationIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct TraceActivation {
        #[rasn(identifier = "traceID")]
        pub trace_id: TraceID,
        #[rasn(identifier = "interfacesToTrace")]
        pub interfaces_to_trace: InterfacesToTrace,
        #[rasn(identifier = "traceDepth")]
        pub trace_depth: TraceDepth,
        #[rasn(identifier = "traceCollectionEntityIPAddress")]
        pub trace_collection_entity_ipaddress: TransportLayerAddress,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<TraceActivationIEExtensions>,
    }
    impl TraceActivation {
        pub fn new(
            trace_id: TraceID,
            interfaces_to_trace: InterfacesToTrace,
            trace_depth: TraceDepth,
            trace_collection_entity_ipaddress: TransportLayerAddress,
            i_e_extensions: Option<TraceActivationIEExtensions>,
        ) -> Self {
            Self {
                trace_id,
                interfaces_to_trace,
                trace_depth,
                trace_collection_entity_ipaddress,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum TraceDepth {
        minimum = 0,
        medium = 1,
        maximum = 2,
        minimumWithoutVendorSpecificExtension = 3,
        mediumWithoutVendorSpecificExtension = 4,
        maximumWithoutVendorSpecificExtension = 5,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct TraceID(pub FixedOctetString<8usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255", extensible))]
    pub struct TransactionID(pub Integer);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousTransportLayerAddressInfoIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousTransportLayerAddressInfoIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct TransportLayerAddressInfoIEExtensions(
        pub SequenceOf<AnonymousTransportLayerAddressInfoIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Transport-Layer-Address-Info")]
    #[non_exhaustive]
    pub struct TransportLayerAddressInfo {
        #[rasn(identifier = "transport-UP-Layer-Addresses-Info-To-Add-List")]
        pub transport_up_layer_addresses_info_to_add_list:
            Option<TransportUPLayerAddressesInfoToAddList>,
        #[rasn(identifier = "transport-UP-Layer-Addresses-Info-To-Remove-List")]
        pub transport_up_layer_addresses_info_to_remove_list:
            Option<TransportUPLayerAddressesInfoToRemoveList>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<TransportLayerAddressInfoIEExtensions>,
    }
    impl TransportLayerAddressInfo {
        pub fn new(
            transport_up_layer_addresses_info_to_add_list: Option<
                TransportUPLayerAddressesInfoToAddList,
            >,
            transport_up_layer_addresses_info_to_remove_list: Option<
                TransportUPLayerAddressesInfoToRemoveList,
            >,
            i_e_extensions: Option<TransportLayerAddressInfoIEExtensions>,
        ) -> Self {
            Self {
                transport_up_layer_addresses_info_to_add_list,
                transport_up_layer_addresses_info_to_remove_list,
                i_e_extensions,
            }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousTransportUPLayerAddressesInfoToAddItemIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousTransportUPLayerAddressesInfoToAddItemIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct TransportUPLayerAddressesInfoToAddItemIEExtensions(
        pub SequenceOf<AnonymousTransportUPLayerAddressesInfoToAddItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "Transport-UP-Layer-Addresses-Info-To-Add-Item"
    )]
    #[non_exhaustive]
    pub struct TransportUPLayerAddressesInfoToAddItem {
        #[rasn(identifier = "iP-SecTransportLayerAddress")]
        pub i_p_sec_transport_layer_address: TransportLayerAddress,
        #[rasn(identifier = "gTPTransportLayerAddressesToAdd")]
        pub g_tptransport_layer_addresses_to_add: Option<GTPTLAs>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<TransportUPLayerAddressesInfoToAddItemIEExtensions>,
    }
    impl TransportUPLayerAddressesInfoToAddItem {
        pub fn new(
            i_p_sec_transport_layer_address: TransportLayerAddress,
            g_tptransport_layer_addresses_to_add: Option<GTPTLAs>,
            i_e_extensions: Option<TransportUPLayerAddressesInfoToAddItemIEExtensions>,
        ) -> Self {
            Self {
                i_p_sec_transport_layer_address,
                g_tptransport_layer_addresses_to_add,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=16"),
        identifier = "Transport-UP-Layer-Addresses-Info-To-Add-List"
    )]
    pub struct TransportUPLayerAddressesInfoToAddList(
        pub SequenceOf<TransportUPLayerAddressesInfoToAddItem>,
    );
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousTransportUPLayerAddressesInfoToRemoveItemIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousTransportUPLayerAddressesInfoToRemoveItemIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct TransportUPLayerAddressesInfoToRemoveItemIEExtensions(
        pub SequenceOf<AnonymousTransportUPLayerAddressesInfoToRemoveItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "Transport-UP-Layer-Addresses-Info-To-Remove-Item"
    )]
    #[non_exhaustive]
    pub struct TransportUPLayerAddressesInfoToRemoveItem {
        #[rasn(identifier = "iP-SecTransportLayerAddress")]
        pub i_p_sec_transport_layer_address: TransportLayerAddress,
        #[rasn(identifier = "gTPTransportLayerAddressesToRemove")]
        pub g_tptransport_layer_addresses_to_remove: Option<GTPTLAs>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<TransportUPLayerAddressesInfoToRemoveItemIEExtensions>,
    }
    impl TransportUPLayerAddressesInfoToRemoveItem {
        pub fn new(
            i_p_sec_transport_layer_address: TransportLayerAddress,
            g_tptransport_layer_addresses_to_remove: Option<GTPTLAs>,
            i_e_extensions: Option<TransportUPLayerAddressesInfoToRemoveItemIEExtensions>,
        ) -> Self {
            Self {
                i_p_sec_transport_layer_address,
                g_tptransport_layer_addresses_to_remove,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=16"),
        identifier = "Transport-UP-Layer-Addresses-Info-To-Remove-List"
    )]
    pub struct TransportUPLayerAddressesInfoToRemoveList(
        pub SequenceOf<TransportUPLayerAddressesInfoToRemoveItem>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=160", extensible))]
    pub struct TransportLayerAddress(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum TypeOfError {
        #[rasn(identifier = "not-understood")]
        not_understood = 0,
        missing = 1,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum UDCParametersContinueUDC {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousUDCParametersIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousUDCParametersIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct UDCParametersIEExtensions(pub SequenceOf<AnonymousUDCParametersIEExtensions>);
    #[doc = " U"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "UDC-Parameters")]
    pub struct UDCParameters {
        #[rasn(identifier = "bufferSize")]
        pub buffer_size: BufferSize,
        pub dictionary: Option<Dictionary>,
        #[rasn(identifier = "continueUDC")]
        pub continue_udc: Option<UDCParametersContinueUDC>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<UDCParametersIEExtensions>,
    }
    impl UDCParameters {
        pub fn new(
            buffer_size: BufferSize,
            dictionary: Option<Dictionary>,
            continue_udc: Option<UDCParametersContinueUDC>,
            i_e_extensions: Option<UDCParametersIEExtensions>,
        ) -> Self {
            Self {
                buffer_size,
                dictionary,
                continue_udc,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "UE-Activity")]
    #[non_exhaustive]
    pub enum UEActivity {
        active = 0,
        #[rasn(identifier = "not-active")]
        not_active = 1,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousUEAssociatedLogicalE1ConnectionItemIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousUEAssociatedLogicalE1ConnectionItemIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct UEAssociatedLogicalE1ConnectionItemIEExtensions(
        pub SequenceOf<AnonymousUEAssociatedLogicalE1ConnectionItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "UE-associatedLogicalE1-ConnectionItem")]
    #[non_exhaustive]
    pub struct UEAssociatedLogicalE1ConnectionItem {
        #[rasn(identifier = "gNB-CU-CP-UE-E1AP-ID")]
        pub g_nb_cu_cp_ue_e1_ap_id: Option<GNBCUCPUEE1APID>,
        #[rasn(identifier = "gNB-CU-UP-UE-E1AP-ID")]
        pub g_nb_cu_up_ue_e1_ap_id: Option<GNBCUUPUEE1APID>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<UEAssociatedLogicalE1ConnectionItemIEExtensions>,
    }
    impl UEAssociatedLogicalE1ConnectionItem {
        pub fn new(
            g_nb_cu_cp_ue_e1_ap_id: Option<GNBCUCPUEE1APID>,
            g_nb_cu_up_ue_e1_ap_id: Option<GNBCUUPUEE1APID>,
            i_e_extensions: Option<UEAssociatedLogicalE1ConnectionItemIEExtensions>,
        ) -> Self {
            Self {
                g_nb_cu_cp_ue_e1_ap_id,
                g_nb_cu_up_ue_e1_ap_id,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=7200", extensible))]
    pub struct UEInactivityInformation(pub Integer);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousUESliceMaximumBitRateItemIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousUESliceMaximumBitRateItemIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct UESliceMaximumBitRateItemIEExtensions(
        pub SequenceOf<AnonymousUESliceMaximumBitRateItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UESliceMaximumBitRateItem {
        #[rasn(identifier = "sNSSAI")]
        pub s_nssai: SNSSAI,
        #[rasn(identifier = "uESliceMaximumBitRateDL")]
        pub u_eslice_maximum_bit_rate_dl: BitRate,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<UESliceMaximumBitRateItemIEExtensions>,
    }
    impl UESliceMaximumBitRateItem {
        pub fn new(
            s_nssai: SNSSAI,
            u_eslice_maximum_bit_rate_dl: BitRate,
            i_e_extensions: Option<UESliceMaximumBitRateItemIEExtensions>,
        ) -> Self {
            Self {
                s_nssai,
                u_eslice_maximum_bit_rate_dl,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8"))]
    pub struct UESliceMaximumBitRateList(pub SequenceOf<UESliceMaximumBitRateItem>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "UL-Configuration")]
    #[non_exhaustive]
    pub enum ULConfiguration {
        #[rasn(identifier = "no-data")]
        no_data = 0,
        shared = 1,
        only = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ULDataSplitThreshold {
        b0 = 0,
        b100 = 1,
        b200 = 2,
        b400 = 3,
        b800 = 4,
        b1600 = 5,
        b3200 = 6,
        b6400 = 7,
        b12800 = 8,
        b25600 = 9,
        b51200 = 10,
        b102400 = 11,
        b204800 = 12,
        b409600 = 13,
        b819200 = 14,
        b1228800 = 15,
        b1638400 = 16,
        b2457600 = 17,
        b3276800 = 18,
        b4096000 = 19,
        b4915200 = 20,
        b5734400 = 21,
        b6553600 = 22,
        infinity = 23,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousULUPTNLAddressToUpdateItemIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousULUPTNLAddressToUpdateItemIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct ULUPTNLAddressToUpdateItemIEExtensions(
        pub SequenceOf<AnonymousULUPTNLAddressToUpdateItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ULUPTNLAddressToUpdateItem {
        #[rasn(identifier = "oldTNLAdress")]
        pub old_tnladress: TransportLayerAddress,
        #[rasn(identifier = "newTNLAdress")]
        pub new_tnladress: TransportLayerAddress,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ULUPTNLAddressToUpdateItemIEExtensions>,
    }
    impl ULUPTNLAddressToUpdateItem {
        pub fn new(
            old_tnladress: TransportLayerAddress,
            new_tnladress: TransportLayerAddress,
            i_e_extensions: Option<ULUPTNLAddressToUpdateItemIEExtensions>,
        ) -> Self {
            Self {
                old_tnladress,
                new_tnladress,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8"), identifier = "UP-Parameters")]
    pub struct UPParameters(pub SequenceOf<UPParametersItem>);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousUPParametersItemIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousUPParametersItemIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct UPParametersItemIEExtensions(pub SequenceOf<AnonymousUPParametersItemIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "UP-Parameters-Item")]
    #[non_exhaustive]
    pub struct UPParametersItem {
        #[rasn(identifier = "uP-TNL-Information")]
        pub u_p_tnl_information: UPTNLInformation,
        #[rasn(identifier = "cell-Group-ID")]
        pub cell_group_id: CellGroupID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<UPParametersItemIEExtensions>,
    }
    impl UPParametersItem {
        pub fn new(
            u_p_tnl_information: UPTNLInformation,
            cell_group_id: CellGroupID,
            i_e_extensions: Option<UPParametersItemIEExtensions>,
        ) -> Self {
            Self {
                u_p_tnl_information,
                cell_group_id,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct UPTNLInformationChoiceExtension {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl UPTNLInformationChoiceExtension {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags, identifier = "UP-TNL-Information")]
    pub enum UPTNLInformation {
        gTPTunnel(GTPTunnel),
        #[rasn(identifier = "choice-extension")]
        choice_extension(UPTNLInformationChoiceExtension),
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousUPSecuritykeyIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousUPSecuritykeyIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct UPSecuritykeyIEExtensions(pub SequenceOf<AnonymousUPSecuritykeyIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UPSecuritykey {
        #[rasn(identifier = "encryptionKey")]
        pub encryption_key: EncryptionKey,
        #[rasn(identifier = "integrityProtectionKey")]
        pub integrity_protection_key: Option<IntegrityProtectionKey>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<UPSecuritykeyIEExtensions>,
    }
    impl UPSecuritykey {
        pub fn new(
            encryption_key: EncryptionKey,
            integrity_protection_key: Option<IntegrityProtectionKey>,
            i_e_extensions: Option<UPSecuritykeyIEExtensions>,
        ) -> Self {
            Self {
                encryption_key,
                integrity_protection_key,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct URIaddress(pub VisibleString);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum UplinkOnlyROHCContinueROHC {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousUplinkOnlyROHCIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousUplinkOnlyROHCIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct UplinkOnlyROHCIEExtensions(pub SequenceOf<AnonymousUplinkOnlyROHCIEExtensions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct UplinkOnlyROHC {
        #[rasn(value("0..=16383", extensible), identifier = "maxCID")]
        pub max_cid: Integer,
        #[rasn(value("0..=511", extensible), identifier = "rOHC-Profiles")]
        pub r_ohc_profiles: Integer,
        #[rasn(identifier = "continueROHC")]
        pub continue_rohc: Option<UplinkOnlyROHCContinueROHC>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<UplinkOnlyROHCIEExtensions>,
    }
    impl UplinkOnlyROHC {
        pub fn new(
            max_cid: Integer,
            r_ohc_profiles: Integer,
            continue_rohc: Option<UplinkOnlyROHCContinueROHC>,
            i_e_extensions: Option<UplinkOnlyROHCIEExtensions>,
        ) -> Self {
            Self {
                max_cid,
                r_ohc_profiles,
                continue_rohc,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum UserPlaneErrorIndicator {
        #[rasn(identifier = "gTP-U-error-indication-received")]
        gTP_U_error_indication_received = 0,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousUserPlaneFailureIndicationIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousUserPlaneFailureIndicationIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct UserPlaneFailureIndicationIEExtensions(
        pub SequenceOf<AnonymousUserPlaneFailureIndicationIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UserPlaneFailureIndication {
        #[rasn(identifier = "userPlaneFailureType")]
        pub user_plane_failure_type: UserPlaneFailureType,
        #[rasn(identifier = "nG-DL-UP-TNL-Information")]
        pub n_g_dl_up_tnl_information: UPTNLInformation,
        #[rasn(identifier = "nG-UL-UP-TNL-Information")]
        pub n_g_ul_up_tnl_information: UPTNLInformation,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<UserPlaneFailureIndicationIEExtensions>,
    }
    impl UserPlaneFailureIndication {
        pub fn new(
            user_plane_failure_type: UserPlaneFailureType,
            n_g_dl_up_tnl_information: UPTNLInformation,
            n_g_ul_up_tnl_information: UPTNLInformation,
            i_e_extensions: Option<UserPlaneFailureIndicationIEExtensions>,
        ) -> Self {
            Self {
                user_plane_failure_type,
                n_g_dl_up_tnl_information,
                n_g_ul_up_tnl_information,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum UserPlaneFailureType {
        #[rasn(identifier = "gtp-u-error-indication-received")]
        gtp_u_error_indication_received = 0,
        #[rasn(identifier = "up-path-failure")]
        up_path_failure = 1,
    }
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod e1_ap_pdu_contents {
    extern crate alloc;
    use super::e1_ap_common_data_types::{ProtocolIEID,Criticality, PrivateIEID};
    use super::e1_ap_constants::{
        ID_ACTIVITY_INFORMATION, ID_ACTIVITY_NOTIFICATION_LEVEL, ID_ADDITIONAL_HANDOVER_INFO,
        ID_ADDITIONAL_RRMPRIORITY_INDEX, ID_ASSOCIATED_SESSION_ID, ID_BCBEARER_CONTEXT_TO_MODIFY,
        ID_BCBEARER_CONTEXT_TO_MODIFY_CONFIRM, ID_BCBEARER_CONTEXT_TO_MODIFY_REQUIRED,
        ID_BCBEARER_CONTEXT_TO_MODIFY_RESPONSE, ID_BCBEARER_CONTEXT_TO_SETUP,
        ID_BCBEARER_CONTEXT_TO_SETUP_RESPONSE, ID_BEARER_CONTEXT_STATUS_CHANGE, ID_CAUSE,
        ID_CHOINITIATION, ID_CNSUPPORT, ID_CRITICALITY_DIAGNOSTICS, ID_DATA_DISCARD_REQUIRED,
        ID_DATA_USAGE_REPORT_LIST, ID_DIRECT_FORWARDING_PATH_AVAILABILITY,
        ID_DLUPTNLADDRESS_TO_UPDATE_LIST, ID_DRBS_SUBJECT_TO_COUNTER_CHECK_LIST_EUTRAN,
        ID_DRBS_SUBJECT_TO_COUNTER_CHECK_LIST_NG_RAN, ID_DRBS_SUBJECT_TO_EARLY_FORWARDING_LIST,
        ID_DRB_CONFIRM_MODIFIED_LIST_EUTRAN, ID_DRB_FAILED_LIST_EUTRAN,
        ID_DRB_FAILED_MOD_LIST_EUTRAN, ID_DRB_FAILED_TO_MODIFY_LIST_EUTRAN,
        ID_DRB_MEASUREMENT_RESULTS_INFORMATION_LIST, ID_DRB_MODIFIED_LIST_EUTRAN,
        ID_DRB_REQUIRED_TO_MODIFY_LIST_EUTRAN, ID_DRB_REQUIRED_TO_REMOVE_LIST_EUTRAN,
        ID_DRB_SETUP_LIST_EUTRAN, ID_DRB_SETUP_MOD_LIST_EUTRAN, ID_DRB_STATUS_LIST,
        ID_DRB_TO_MODIFY_LIST_EUTRAN, ID_DRB_TO_REMOVE_LIST_EUTRAN, ID_DRB_TO_SETUP_LIST_EUTRAN,
        ID_DRB_TO_SETUP_MOD_LIST_EUTRAN, ID_ECGI_SUPPORT_LIST, ID_EXTENDED_GNB_CU_CP_NAME,
        ID_EXTENDED_GNB_CU_UP_NAME, ID_EXTENDED_NR_CGI_SUPPORT_LIST,
        ID_EXTENDED_SLICE_SUPPORT_LIST, ID_GLOBAL_MBSSESSION_ID, ID_GNB_CU_CP_MBS_E1_AP_ID,
        ID_GNB_CU_CP_TNLA_FAILED_TO_SETUP_LIST, ID_GNB_CU_CP_TNLA_SETUP_LIST,
        ID_GNB_CU_CP_TNLA_TO_ADD_LIST, ID_GNB_CU_CP_TNLA_TO_REMOVE_LIST,
        ID_GNB_CU_CP_TNLA_TO_UPDATE_LIST, ID_GNB_CU_UP_MBS_E1_AP_ID,
        ID_GNB_CU_UP_OVERLOAD_INFORMATION, ID_GNB_CU_UP_TNLA_TO_REMOVE_LIST, ID_GNB_DU_ID,
        ID_G_NB_CU_CP_MEASUREMENT_ID, ID_G_NB_CU_CP_NAME, ID_G_NB_CU_CP_UE_E1_AP_ID,
        ID_G_NB_CU_UP_CAPACITY, ID_G_NB_CU_UP_ID, ID_G_NB_CU_UP_MBS_SUPPORT_INFO,
        ID_G_NB_CU_UP_MEASUREMENT_ID, ID_G_NB_CU_UP_NAME, ID_G_NB_CU_UP_UE_E1_AP_ID,
        ID_HW_CAPACITY_INDICATOR, ID_IAB_DONOR_CU_UPPSKINFO, ID_INACTIVITY_INFORMATION_REQUEST,
        ID_MANAGEMENT_BASED_MDTPLMNLIST, ID_MANAGEMENT_BASED_MDTPLMNMODIFICATION_LIST,
        ID_MBSMULTICAST_F1_UCONTEXT_DESCRIPTOR, ID_MBSSESSION_RESOURCE_NOTIFICATION,
        ID_MBS_SERVICE_AREA, ID_MCBEARER_CONTEXT_TO_MODIFY, ID_MCBEARER_CONTEXT_TO_MODIFY_CONFIRM,
        ID_MCBEARER_CONTEXT_TO_MODIFY_REQUIRED, ID_MCBEARER_CONTEXT_TO_MODIFY_RESPONSE,
        ID_MCBEARER_CONTEXT_TO_SETUP, ID_MCBEARER_CONTEXT_TO_SETUP_RESPONSE,
        ID_MDTPOLLUTED_MEASUREMENT_INDICATOR, ID_MT_SDT_INFORMATION, ID_MT_SDT_INFORMATION_REQUEST,
        ID_NEW_UL_TNL_INFORMATION_REQUIRED, ID_NPNCONTEXT_INFO, ID_NPNSUPPORT_INFO,
        ID_PDU_SESSION_RESOURCE_CONFIRM_MODIFIED_LIST, ID_PDU_SESSION_RESOURCE_DATA_USAGE_LIST,
        ID_PDU_SESSION_RESOURCE_FAILED_LIST, ID_PDU_SESSION_RESOURCE_FAILED_MOD_LIST,
        ID_PDU_SESSION_RESOURCE_FAILED_TO_MODIFY_LIST, ID_PDU_SESSION_RESOURCE_MODIFIED_LIST,
        ID_PDU_SESSION_RESOURCE_REQUIRED_TO_MODIFY_LIST, ID_PDU_SESSION_RESOURCE_SETUP_LIST,
        ID_PDU_SESSION_RESOURCE_SETUP_MOD_LIST, ID_PDU_SESSION_RESOURCE_TO_MODIFY_LIST,
        ID_PDU_SESSION_RESOURCE_TO_REMOVE_LIST, ID_PDU_SESSION_RESOURCE_TO_SETUP_LIST,
        ID_PDU_SESSION_RESOURCE_TO_SETUP_MOD_LIST, ID_PDU_SESSION_TO_NOTIFY_LIST, ID_PPI,
        ID_PRIVACY_INDICATOR, ID_RANUEID, ID_REGISTRATION_REQUEST, ID_REPORTING_PERIODICITY,
        ID_REPORT_CHARACTERISTICS, ID_RESET_TYPE, ID_RETAINABILITY_MEASUREMENTS_INFO,
        ID_SCGACTIVATION_STATUS, ID_SDTCONTINUE_ROHC, ID_SDT_DATA_SIZE_THRESHOLD,
        ID_SDT_DATA_SIZE_THRESHOLD_CROSSED, ID_SECURITY_INFORMATION, ID_SERVING_PLMN,
        ID_SUBSCRIBER_PROFILE_IDFOR_RFP, ID_SUPPORTED_PLMNS,
        ID_SYSTEM_BEARER_CONTEXT_MODIFICATION_CONFIRM,
        ID_SYSTEM_BEARER_CONTEXT_MODIFICATION_REQUEST,
        ID_SYSTEM_BEARER_CONTEXT_MODIFICATION_REQUIRED,
        ID_SYSTEM_BEARER_CONTEXT_MODIFICATION_RESPONSE, ID_SYSTEM_BEARER_CONTEXT_SETUP_REQUEST,
        ID_SYSTEM_BEARER_CONTEXT_SETUP_RESPONSE, ID_SYSTEM_GNB_CU_UP_COUNTER_CHECK_REQUEST,
        ID_TIME_TO_WAIT, ID_TNL_AVAILABLE_CAPACITY_INDICATOR, ID_TRACE_ACTIVATION,
        ID_TRACE_COLLECTION_ENTITY_IPADDRESS, ID_TRACE_ID, ID_TRANSACTION_ID,
        ID_TRANSPORT_LAYER_ADDRESS_INFO, ID_UEDLAGGREGATE_MAXIMUM_BIT_RATE,
        ID_UEDLMAXIMUM_INTEGRITY_PROTECTED_DATA_RATE, ID_UEINACTIVITY_INFORMATION,
        ID_UESLICE_MAXIMUM_BIT_RATE_LIST, ID_UE_ASSOCIATED_LOGICAL_E1_CONNECTION_ITEM,
        ID_UE_ASSOCIATED_LOGICAL_E1_CONNECTION_LIST_RES_ACK, ID_UE_INACTIVITY_TIMER,
        ID_ULUPTNLADDRESS_TO_UPDATE_LIST, ID_URIADDRESS, MAXNOOF_DRBS, MAXNOOF_ERRORS,
        MAXNOOF_INDIVIDUAL_E1_CONNECTIONS_TO_RESET, MAXNOOF_PSKS, MAXNOOF_SPLMNS,
        MAXNOOF_TNLADDRESSES, MAXNOOF_TNLASSOCIATIONS,
    };
    use super::e1_ap_containers::{
        PrivateIEContainer, ProtocolExtensionContainer, ProtocolIEContainer,
        ProtocolIEContainerList, ProtocolIESingleContainer, E1APPRIVATEIES, E1APPROTOCOLEXTENSION,
        E1APPROTOCOLIES,
    };
    use super::e1_ap_ies::*;
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBCBearerContextModificationConfirmProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBCBearerContextModificationConfirmProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBCBearerContextModificationConfirmProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousBCBearerContextModificationConfirmProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousBCBearerContextModificationConfirmProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct BCBearerContextModificationConfirmProtocolIEs(
        pub SequenceOf<AnonymousBCBearerContextModificationConfirmProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " BC BEARER CONTEXT MODIFICATION CONFIRM"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BCBearerContextModificationConfirm {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: BCBearerContextModificationConfirmProtocolIEs,
    }
    impl BCBearerContextModificationConfirm {
        pub fn new(protocol_ies: BCBearerContextModificationConfirmProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBCBearerContextModificationFailureProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBCBearerContextModificationFailureProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBCBearerContextModificationFailureProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousBCBearerContextModificationFailureProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousBCBearerContextModificationFailureProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct BCBearerContextModificationFailureProtocolIEs(
        pub SequenceOf<AnonymousBCBearerContextModificationFailureProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " BC BEARER CONTEXT MODIFICATION FAILURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BCBearerContextModificationFailure {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: BCBearerContextModificationFailureProtocolIEs,
    }
    impl BCBearerContextModificationFailure {
        pub fn new(protocol_ies: BCBearerContextModificationFailureProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBCBearerContextModificationRequestProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBCBearerContextModificationRequestProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBCBearerContextModificationRequestProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousBCBearerContextModificationRequestProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousBCBearerContextModificationRequestProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct BCBearerContextModificationRequestProtocolIEs(
        pub SequenceOf<AnonymousBCBearerContextModificationRequestProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " BC BEARER CONTEXT MODIFICATION"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " BC BEARER CONTEXT MODIFICATION REQUEST"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BCBearerContextModificationRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: BCBearerContextModificationRequestProtocolIEs,
    }
    impl BCBearerContextModificationRequest {
        pub fn new(protocol_ies: BCBearerContextModificationRequestProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBCBearerContextModificationRequiredProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBCBearerContextModificationRequiredProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBCBearerContextModificationRequiredProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousBCBearerContextModificationRequiredProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousBCBearerContextModificationRequiredProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct BCBearerContextModificationRequiredProtocolIEs(
        pub SequenceOf<AnonymousBCBearerContextModificationRequiredProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " BC BEARER CONTEXT MODIFICATION REQUIRED"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " BC BEARER CONTEXT MODIFICATION REQUIRED"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BCBearerContextModificationRequired {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: BCBearerContextModificationRequiredProtocolIEs,
    }
    impl BCBearerContextModificationRequired {
        pub fn new(protocol_ies: BCBearerContextModificationRequiredProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBCBearerContextModificationResponseProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBCBearerContextModificationResponseProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBCBearerContextModificationResponseProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousBCBearerContextModificationResponseProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousBCBearerContextModificationResponseProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct BCBearerContextModificationResponseProtocolIEs(
        pub SequenceOf<AnonymousBCBearerContextModificationResponseProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " BC BEARER CONTEXT MODIFICATION RESPONSE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BCBearerContextModificationResponse {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: BCBearerContextModificationResponseProtocolIEs,
    }
    impl BCBearerContextModificationResponse {
        pub fn new(protocol_ies: BCBearerContextModificationResponseProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBCBearerContextReleaseCommandProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBCBearerContextReleaseCommandProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBCBearerContextReleaseCommandProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousBCBearerContextReleaseCommandProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousBCBearerContextReleaseCommandProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct BCBearerContextReleaseCommandProtocolIEs(
        pub SequenceOf<AnonymousBCBearerContextReleaseCommandProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " BC BEARER CONTEXT RELEASE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " BC BEARER CONTEXT RELEASE COMMAND"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BCBearerContextReleaseCommand {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: BCBearerContextReleaseCommandProtocolIEs,
    }
    impl BCBearerContextReleaseCommand {
        pub fn new(protocol_ies: BCBearerContextReleaseCommandProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBCBearerContextReleaseCompleteProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBCBearerContextReleaseCompleteProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBCBearerContextReleaseCompleteProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousBCBearerContextReleaseCompleteProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousBCBearerContextReleaseCompleteProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct BCBearerContextReleaseCompleteProtocolIEs(
        pub SequenceOf<AnonymousBCBearerContextReleaseCompleteProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " BC BEARER CONTEXT RELEASE COMPLETE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BCBearerContextReleaseComplete {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: BCBearerContextReleaseCompleteProtocolIEs,
    }
    impl BCBearerContextReleaseComplete {
        pub fn new(protocol_ies: BCBearerContextReleaseCompleteProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBCBearerContextReleaseRequestProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBCBearerContextReleaseRequestProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBCBearerContextReleaseRequestProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousBCBearerContextReleaseRequestProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousBCBearerContextReleaseRequestProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct BCBearerContextReleaseRequestProtocolIEs(
        pub SequenceOf<AnonymousBCBearerContextReleaseRequestProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " BC BEARER CONTEXT RELEASE REQUEST"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " BC BEARER CONTEXT RELEASE REQUEST"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BCBearerContextReleaseRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: BCBearerContextReleaseRequestProtocolIEs,
    }
    impl BCBearerContextReleaseRequest {
        pub fn new(protocol_ies: BCBearerContextReleaseRequestProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBCBearerContextSetupFailureProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBCBearerContextSetupFailureProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBCBearerContextSetupFailureProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousBCBearerContextSetupFailureProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousBCBearerContextSetupFailureProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct BCBearerContextSetupFailureProtocolIEs(
        pub SequenceOf<AnonymousBCBearerContextSetupFailureProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " BC BEARER CONTEXT SETUP FAILURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BCBearerContextSetupFailure {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: BCBearerContextSetupFailureProtocolIEs,
    }
    impl BCBearerContextSetupFailure {
        pub fn new(protocol_ies: BCBearerContextSetupFailureProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBCBearerContextSetupRequestProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBCBearerContextSetupRequestProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBCBearerContextSetupRequestProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousBCBearerContextSetupRequestProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousBCBearerContextSetupRequestProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct BCBearerContextSetupRequestProtocolIEs(
        pub SequenceOf<AnonymousBCBearerContextSetupRequestProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " BC BEARER CONTEXT SETUP"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " BC BEARER CONTEXT SETUP REQUEST"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BCBearerContextSetupRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: BCBearerContextSetupRequestProtocolIEs,
    }
    impl BCBearerContextSetupRequest {
        pub fn new(protocol_ies: BCBearerContextSetupRequestProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBCBearerContextSetupResponseProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBCBearerContextSetupResponseProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBCBearerContextSetupResponseProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousBCBearerContextSetupResponseProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousBCBearerContextSetupResponseProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct BCBearerContextSetupResponseProtocolIEs(
        pub SequenceOf<AnonymousBCBearerContextSetupResponseProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " BC BEARER CONTEXT SETUP RESPONSE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BCBearerContextSetupResponse {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: BCBearerContextSetupResponseProtocolIEs,
    }
    impl BCBearerContextSetupResponse {
        pub fn new(protocol_ies: BCBearerContextSetupResponseProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBearerContextInactivityNotificationProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBearerContextInactivityNotificationProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBearerContextInactivityNotificationProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousBearerContextInactivityNotificationProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousBearerContextInactivityNotificationProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct BearerContextInactivityNotificationProtocolIEs(
        pub SequenceOf<AnonymousBearerContextInactivityNotificationProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " BEARER CONTEXT INACTIVITY NOTIFICATION"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Bearer Context Inactivity Notification"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BearerContextInactivityNotification {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: BearerContextInactivityNotificationProtocolIEs,
    }
    impl BearerContextInactivityNotification {
        pub fn new(protocol_ies: BearerContextInactivityNotificationProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBearerContextModificationConfirmProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBearerContextModificationConfirmProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBearerContextModificationConfirmProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousBearerContextModificationConfirmProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousBearerContextModificationConfirmProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct BearerContextModificationConfirmProtocolIEs(
        pub SequenceOf<AnonymousBearerContextModificationConfirmProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Bearer Context Modification Confirm"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BearerContextModificationConfirm {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: BearerContextModificationConfirmProtocolIEs,
    }
    impl BearerContextModificationConfirm {
        pub fn new(protocol_ies: BearerContextModificationConfirmProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBearerContextModificationFailureProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBearerContextModificationFailureProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBearerContextModificationFailureProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousBearerContextModificationFailureProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousBearerContextModificationFailureProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct BearerContextModificationFailureProtocolIEs(
        pub SequenceOf<AnonymousBearerContextModificationFailureProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Bearer Context Modification Failure"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BearerContextModificationFailure {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: BearerContextModificationFailureProtocolIEs,
    }
    impl BearerContextModificationFailure {
        pub fn new(protocol_ies: BearerContextModificationFailureProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBearerContextModificationRequestProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBearerContextModificationRequestProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBearerContextModificationRequestProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousBearerContextModificationRequestProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousBearerContextModificationRequestProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct BearerContextModificationRequestProtocolIEs(
        pub SequenceOf<AnonymousBearerContextModificationRequestProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " BEARER CONTEXT MODIFICATION"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Bearer Context Modification Request"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BearerContextModificationRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: BearerContextModificationRequestProtocolIEs,
    }
    impl BearerContextModificationRequest {
        pub fn new(protocol_ies: BearerContextModificationRequestProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBearerContextModificationRequiredProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBearerContextModificationRequiredProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBearerContextModificationRequiredProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousBearerContextModificationRequiredProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousBearerContextModificationRequiredProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct BearerContextModificationRequiredProtocolIEs(
        pub SequenceOf<AnonymousBearerContextModificationRequiredProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " BEARER CONTEXT MODIFICATION REQUIRED"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Bearer Context Modification Required"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BearerContextModificationRequired {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: BearerContextModificationRequiredProtocolIEs,
    }
    impl BearerContextModificationRequired {
        pub fn new(protocol_ies: BearerContextModificationRequiredProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBearerContextModificationResponseProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBearerContextModificationResponseProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBearerContextModificationResponseProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousBearerContextModificationResponseProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousBearerContextModificationResponseProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct BearerContextModificationResponseProtocolIEs(
        pub SequenceOf<AnonymousBearerContextModificationResponseProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Bearer Context Modification Response"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BearerContextModificationResponse {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: BearerContextModificationResponseProtocolIEs,
    }
    impl BearerContextModificationResponse {
        pub fn new(protocol_ies: BearerContextModificationResponseProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBearerContextReleaseCommandProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBearerContextReleaseCommandProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBearerContextReleaseCommandProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousBearerContextReleaseCommandProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousBearerContextReleaseCommandProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct BearerContextReleaseCommandProtocolIEs(
        pub SequenceOf<AnonymousBearerContextReleaseCommandProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " BEARER CONTEXT RELEASE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Bearer Context Release Command"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BearerContextReleaseCommand {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: BearerContextReleaseCommandProtocolIEs,
    }
    impl BearerContextReleaseCommand {
        pub fn new(protocol_ies: BearerContextReleaseCommandProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBearerContextReleaseCompleteProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBearerContextReleaseCompleteProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBearerContextReleaseCompleteProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousBearerContextReleaseCompleteProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousBearerContextReleaseCompleteProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct BearerContextReleaseCompleteProtocolIEs(
        pub SequenceOf<AnonymousBearerContextReleaseCompleteProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Bearer Context Release Complete"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BearerContextReleaseComplete {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: BearerContextReleaseCompleteProtocolIEs,
    }
    impl BearerContextReleaseComplete {
        pub fn new(protocol_ies: BearerContextReleaseCompleteProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBearerContextReleaseRequestProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBearerContextReleaseRequestProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBearerContextReleaseRequestProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousBearerContextReleaseRequestProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousBearerContextReleaseRequestProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct BearerContextReleaseRequestProtocolIEs(
        pub SequenceOf<AnonymousBearerContextReleaseRequestProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " BEARER CONTEXT RELEASE REQUEST"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Bearer Context Release Request"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BearerContextReleaseRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: BearerContextReleaseRequestProtocolIEs,
    }
    impl BearerContextReleaseRequest {
        pub fn new(protocol_ies: BearerContextReleaseRequestProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBearerContextSetupFailureProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBearerContextSetupFailureProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBearerContextSetupFailureProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousBearerContextSetupFailureProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousBearerContextSetupFailureProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct BearerContextSetupFailureProtocolIEs(
        pub SequenceOf<AnonymousBearerContextSetupFailureProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Bearer Context Setup Failure"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BearerContextSetupFailure {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: BearerContextSetupFailureProtocolIEs,
    }
    impl BearerContextSetupFailure {
        pub fn new(protocol_ies: BearerContextSetupFailureProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBearerContextSetupRequestProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBearerContextSetupRequestProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBearerContextSetupRequestProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousBearerContextSetupRequestProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousBearerContextSetupRequestProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct BearerContextSetupRequestProtocolIEs(
        pub SequenceOf<AnonymousBearerContextSetupRequestProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " BEARER CONTEXT SETUP"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Bearer Context Setup Request"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BearerContextSetupRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: BearerContextSetupRequestProtocolIEs,
    }
    impl BearerContextSetupRequest {
        pub fn new(protocol_ies: BearerContextSetupRequestProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousBearerContextSetupResponseProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousBearerContextSetupResponseProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousBearerContextSetupResponseProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousBearerContextSetupResponseProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousBearerContextSetupResponseProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct BearerContextSetupResponseProtocolIEs(
        pub SequenceOf<AnonymousBearerContextSetupResponseProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Bearer Context Setup Response"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BearerContextSetupResponse {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: BearerContextSetupResponseProtocolIEs,
    }
    impl BearerContextSetupResponse {
        pub fn new(protocol_ies: BearerContextSetupResponseProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousCellTrafficTraceProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousCellTrafficTraceProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousCellTrafficTraceProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousCellTrafficTraceProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousCellTrafficTraceProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct CellTrafficTraceProtocolIEs(pub SequenceOf<AnonymousCellTrafficTraceProtocolIEs>);
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " CELL TRAFFIC TRACE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct CellTrafficTrace {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: CellTrafficTraceProtocolIEs,
    }
    impl CellTrafficTrace {
        pub fn new(protocol_ies: CellTrafficTraceProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDLDataNotificationProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDLDataNotificationProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDLDataNotificationProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousDLDataNotificationProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousDLDataNotificationProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct DLDataNotificationProtocolIEs(
        pub SequenceOf<AnonymousDLDataNotificationProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " DL DATA NOTIFICATION"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " DL Data Notification"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct DLDataNotification {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: DLDataNotificationProtocolIEs,
    }
    impl DLDataNotification {
        pub fn new(protocol_ies: DLDataNotificationProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8"))]
    pub struct DLUPTNLAddressToUpdateList(pub SequenceOf<DLUPTNLAddressToUpdateItem>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "DRB-Status-List")]
    pub struct DRBStatusList(pub SequenceOf<DRBStatusItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDataUsageReportProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDataUsageReportProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDataUsageReportProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousDataUsageReportProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousDataUsageReportProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct DataUsageReportProtocolIEs(pub SequenceOf<AnonymousDataUsageReportProtocolIEs>);
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " DATA USAGE REPORT"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Data Usage Report"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct DataUsageReport {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: DataUsageReportProtocolIEs,
    }
    impl DataUsageReport {
        pub fn new(protocol_ies: DataUsageReportProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousDeactivateTraceProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousDeactivateTraceProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousDeactivateTraceProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousDeactivateTraceProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousDeactivateTraceProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct DeactivateTraceProtocolIEs(pub SequenceOf<AnonymousDeactivateTraceProtocolIEs>);
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " DEACTIVATE TRACE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct DeactivateTrace {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: DeactivateTraceProtocolIEs,
    }
    impl DeactivateTrace {
        pub fn new(protocol_ies: DeactivateTraceProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousE1ReleaseRequestProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousE1ReleaseRequestProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousE1ReleaseRequestProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousE1ReleaseRequestProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousE1ReleaseRequestProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct E1ReleaseRequestProtocolIEs(pub SequenceOf<AnonymousE1ReleaseRequestProtocolIEs>);
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " E1 RELEASE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " E1 Release Request"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct E1ReleaseRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: E1ReleaseRequestProtocolIEs,
    }
    impl E1ReleaseRequest {
        pub fn new(protocol_ies: E1ReleaseRequestProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousE1ReleaseResponseProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousE1ReleaseResponseProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousE1ReleaseResponseProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousE1ReleaseResponseProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousE1ReleaseResponseProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct E1ReleaseResponseProtocolIEs(pub SequenceOf<AnonymousE1ReleaseResponseProtocolIEs>);
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " E1 Release Response"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct E1ReleaseResponse {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: E1ReleaseResponseProtocolIEs,
    }
    impl E1ReleaseResponse {
        pub fn new(protocol_ies: E1ReleaseResponseProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousEarlyForwardingSNTransferProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousEarlyForwardingSNTransferProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousEarlyForwardingSNTransferProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousEarlyForwardingSNTransferProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousEarlyForwardingSNTransferProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct EarlyForwardingSNTransferProtocolIEs(
        pub SequenceOf<AnonymousEarlyForwardingSNTransferProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " EARLY FORWARDING SN TRANSFER"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Early Forwarding SN Transfer"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct EarlyForwardingSNTransfer {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: EarlyForwardingSNTransferProtocolIEs,
    }
    impl EarlyForwardingSNTransfer {
        pub fn new(protocol_ies: EarlyForwardingSNTransferProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousErrorIndicationProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousErrorIndicationProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousErrorIndicationProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousErrorIndicationProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousErrorIndicationProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct ErrorIndicationProtocolIEs(pub SequenceOf<AnonymousErrorIndicationProtocolIEs>);
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " ERROR INDICATION"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ErrorIndication {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ErrorIndicationProtocolIEs,
    }
    impl ErrorIndication {
        pub fn new(protocol_ies: ErrorIndicationProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGNBCUCPConfigurationUpdateProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGNBCUCPConfigurationUpdateProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGNBCUCPConfigurationUpdateProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousGNBCUCPConfigurationUpdateProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousGNBCUCPConfigurationUpdateProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct GNBCUCPConfigurationUpdateProtocolIEs(
        pub SequenceOf<AnonymousGNBCUCPConfigurationUpdateProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " GNB-CU-CP CONFIGURATION UPDATE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " GNB-CU-CP Configuration Update"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GNB-CU-CP-ConfigurationUpdate")]
    #[non_exhaustive]
    pub struct GNBCUCPConfigurationUpdate {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: GNBCUCPConfigurationUpdateProtocolIEs,
    }
    impl GNBCUCPConfigurationUpdate {
        pub fn new(protocol_ies: GNBCUCPConfigurationUpdateProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGNBCUCPConfigurationUpdateAcknowledgeProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGNBCUCPConfigurationUpdateAcknowledgeProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGNBCUCPConfigurationUpdateAcknowledgeProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousGNBCUCPConfigurationUpdateAcknowledgeProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousGNBCUCPConfigurationUpdateAcknowledgeProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct GNBCUCPConfigurationUpdateAcknowledgeProtocolIEs(
        pub SequenceOf<AnonymousGNBCUCPConfigurationUpdateAcknowledgeProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " GNB-CU-CP Configuration Update Acknowledge"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "GNB-CU-CP-ConfigurationUpdateAcknowledge"
    )]
    #[non_exhaustive]
    pub struct GNBCUCPConfigurationUpdateAcknowledge {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: GNBCUCPConfigurationUpdateAcknowledgeProtocolIEs,
    }
    impl GNBCUCPConfigurationUpdateAcknowledge {
        pub fn new(protocol_ies: GNBCUCPConfigurationUpdateAcknowledgeProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGNBCUCPConfigurationUpdateFailureProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGNBCUCPConfigurationUpdateFailureProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGNBCUCPConfigurationUpdateFailureProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousGNBCUCPConfigurationUpdateFailureProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousGNBCUCPConfigurationUpdateFailureProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct GNBCUCPConfigurationUpdateFailureProtocolIEs(
        pub SequenceOf<AnonymousGNBCUCPConfigurationUpdateFailureProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " GNB-CU-CP Configuration Update Failure"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GNB-CU-CP-ConfigurationUpdateFailure")]
    #[non_exhaustive]
    pub struct GNBCUCPConfigurationUpdateFailure {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: GNBCUCPConfigurationUpdateFailureProtocolIEs,
    }
    impl GNBCUCPConfigurationUpdateFailure {
        pub fn new(protocol_ies: GNBCUCPConfigurationUpdateFailureProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGNBCUCPE1SetupFailureProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGNBCUCPE1SetupFailureProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGNBCUCPE1SetupFailureProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousGNBCUCPE1SetupFailureProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousGNBCUCPE1SetupFailureProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct GNBCUCPE1SetupFailureProtocolIEs(
        pub SequenceOf<AnonymousGNBCUCPE1SetupFailureProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " GNB-CU-CP E1 Setup Failure"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GNB-CU-CP-E1SetupFailure")]
    #[non_exhaustive]
    pub struct GNBCUCPE1SetupFailure {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: GNBCUCPE1SetupFailureProtocolIEs,
    }
    impl GNBCUCPE1SetupFailure {
        pub fn new(protocol_ies: GNBCUCPE1SetupFailureProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGNBCUCPE1SetupRequestProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGNBCUCPE1SetupRequestProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGNBCUCPE1SetupRequestProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousGNBCUCPE1SetupRequestProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousGNBCUCPE1SetupRequestProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct GNBCUCPE1SetupRequestProtocolIEs(
        pub SequenceOf<AnonymousGNBCUCPE1SetupRequestProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " GNB-CU-CP E1 SETUP"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " GNB-CU-CP E1 Setup Request"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GNB-CU-CP-E1SetupRequest")]
    #[non_exhaustive]
    pub struct GNBCUCPE1SetupRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: GNBCUCPE1SetupRequestProtocolIEs,
    }
    impl GNBCUCPE1SetupRequest {
        pub fn new(protocol_ies: GNBCUCPE1SetupRequestProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGNBCUCPE1SetupResponseProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGNBCUCPE1SetupResponseProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGNBCUCPE1SetupResponseProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousGNBCUCPE1SetupResponseProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousGNBCUCPE1SetupResponseProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct GNBCUCPE1SetupResponseProtocolIEs(
        pub SequenceOf<AnonymousGNBCUCPE1SetupResponseProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " GNB-CU-CP E1 Setup Response"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GNB-CU-CP-E1SetupResponse")]
    #[non_exhaustive]
    pub struct GNBCUCPE1SetupResponse {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: GNBCUCPE1SetupResponseProtocolIEs,
    }
    impl GNBCUCPE1SetupResponse {
        pub fn new(protocol_ies: GNBCUCPE1SetupResponseProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=32"),
        identifier = "GNB-CU-CP-TNLA-Failed-To-Setup-List"
    )]
    pub struct GNBCUCPTNLAFailedToSetupList(pub SequenceOf<GNBCUCPTNLAFailedToSetupItem>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "GNB-CU-CP-TNLA-Setup-List")]
    pub struct GNBCUCPTNLASetupList(pub SequenceOf<GNBCUCPTNLASetupItem>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "GNB-CU-CP-TNLA-To-Add-List")]
    pub struct GNBCUCPTNLAToAddList(pub SequenceOf<GNBCUCPTNLAToAddItem>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "GNB-CU-CP-TNLA-To-Remove-List")]
    pub struct GNBCUCPTNLAToRemoveList(pub SequenceOf<GNBCUCPTNLAToRemoveItem>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "GNB-CU-CP-TNLA-To-Update-List")]
    pub struct GNBCUCPTNLAToUpdateList(pub SequenceOf<GNBCUCPTNLAToUpdateItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGNBCUCPMeasurementResultsInformationProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGNBCUCPMeasurementResultsInformationProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGNBCUCPMeasurementResultsInformationProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousGNBCUCPMeasurementResultsInformationProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousGNBCUCPMeasurementResultsInformationProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct GNBCUCPMeasurementResultsInformationProtocolIEs(
        pub SequenceOf<AnonymousGNBCUCPMeasurementResultsInformationProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " gNB-CU-CP MEASUREMENT RESULTS INFORMATION"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GNB-CU-CPMeasurementResultsInformation")]
    #[non_exhaustive]
    pub struct GNBCUCPMeasurementResultsInformation {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: GNBCUCPMeasurementResultsInformationProtocolIEs,
    }
    impl GNBCUCPMeasurementResultsInformation {
        pub fn new(protocol_ies: GNBCUCPMeasurementResultsInformationProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGNBCUUPConfigurationUpdateProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGNBCUUPConfigurationUpdateProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGNBCUUPConfigurationUpdateProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousGNBCUUPConfigurationUpdateProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousGNBCUUPConfigurationUpdateProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct GNBCUUPConfigurationUpdateProtocolIEs(
        pub SequenceOf<AnonymousGNBCUUPConfigurationUpdateProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " GNB-CU-UP CONFIGURATION UPDATE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " GNB-CU-UP Configuration Update"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GNB-CU-UP-ConfigurationUpdate")]
    #[non_exhaustive]
    pub struct GNBCUUPConfigurationUpdate {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: GNBCUUPConfigurationUpdateProtocolIEs,
    }
    impl GNBCUUPConfigurationUpdate {
        pub fn new(protocol_ies: GNBCUUPConfigurationUpdateProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGNBCUUPConfigurationUpdateAcknowledgeProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGNBCUUPConfigurationUpdateAcknowledgeProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGNBCUUPConfigurationUpdateAcknowledgeProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousGNBCUUPConfigurationUpdateAcknowledgeProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousGNBCUUPConfigurationUpdateAcknowledgeProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct GNBCUUPConfigurationUpdateAcknowledgeProtocolIEs(
        pub SequenceOf<AnonymousGNBCUUPConfigurationUpdateAcknowledgeProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " GNB-CU-UP Configuration Update Acknowledge"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "GNB-CU-UP-ConfigurationUpdateAcknowledge"
    )]
    #[non_exhaustive]
    pub struct GNBCUUPConfigurationUpdateAcknowledge {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: GNBCUUPConfigurationUpdateAcknowledgeProtocolIEs,
    }
    impl GNBCUUPConfigurationUpdateAcknowledge {
        pub fn new(protocol_ies: GNBCUUPConfigurationUpdateAcknowledgeProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGNBCUUPConfigurationUpdateFailureProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGNBCUUPConfigurationUpdateFailureProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGNBCUUPConfigurationUpdateFailureProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousGNBCUUPConfigurationUpdateFailureProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousGNBCUUPConfigurationUpdateFailureProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct GNBCUUPConfigurationUpdateFailureProtocolIEs(
        pub SequenceOf<AnonymousGNBCUUPConfigurationUpdateFailureProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " GNB-CU-UP Configuration Update Failure"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GNB-CU-UP-ConfigurationUpdateFailure")]
    #[non_exhaustive]
    pub struct GNBCUUPConfigurationUpdateFailure {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: GNBCUUPConfigurationUpdateFailureProtocolIEs,
    }
    impl GNBCUUPConfigurationUpdateFailure {
        pub fn new(protocol_ies: GNBCUUPConfigurationUpdateFailureProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGNBCUUPCounterCheckRequestProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGNBCUUPCounterCheckRequestProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGNBCUUPCounterCheckRequestProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousGNBCUUPCounterCheckRequestProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousGNBCUUPCounterCheckRequestProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct GNBCUUPCounterCheckRequestProtocolIEs(
        pub SequenceOf<AnonymousGNBCUUPCounterCheckRequestProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " GNB-CU-UP COUNTER CHECK"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " gNB-CU-UP Counter Check Request"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GNB-CU-UP-CounterCheckRequest")]
    #[non_exhaustive]
    pub struct GNBCUUPCounterCheckRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: GNBCUUPCounterCheckRequestProtocolIEs,
    }
    impl GNBCUUPCounterCheckRequest {
        pub fn new(protocol_ies: GNBCUUPCounterCheckRequestProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGNBCUUPE1SetupFailureProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGNBCUUPE1SetupFailureProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGNBCUUPE1SetupFailureProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousGNBCUUPE1SetupFailureProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousGNBCUUPE1SetupFailureProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct GNBCUUPE1SetupFailureProtocolIEs(
        pub SequenceOf<AnonymousGNBCUUPE1SetupFailureProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " GNB-CU-UP E1 Setup Failure"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GNB-CU-UP-E1SetupFailure")]
    #[non_exhaustive]
    pub struct GNBCUUPE1SetupFailure {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: GNBCUUPE1SetupFailureProtocolIEs,
    }
    impl GNBCUUPE1SetupFailure {
        pub fn new(protocol_ies: GNBCUUPE1SetupFailureProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGNBCUUPE1SetupRequestProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGNBCUUPE1SetupRequestProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGNBCUUPE1SetupRequestProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousGNBCUUPE1SetupRequestProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousGNBCUUPE1SetupRequestProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct GNBCUUPE1SetupRequestProtocolIEs(
        pub SequenceOf<AnonymousGNBCUUPE1SetupRequestProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " GNB-CU-UP E1 SETUP"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " GNB-CU-UP E1 Setup Request"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GNB-CU-UP-E1SetupRequest")]
    #[non_exhaustive]
    pub struct GNBCUUPE1SetupRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: GNBCUUPE1SetupRequestProtocolIEs,
    }
    impl GNBCUUPE1SetupRequest {
        pub fn new(protocol_ies: GNBCUUPE1SetupRequestProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGNBCUUPE1SetupResponseProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGNBCUUPE1SetupResponseProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGNBCUUPE1SetupResponseProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousGNBCUUPE1SetupResponseProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousGNBCUUPE1SetupResponseProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct GNBCUUPE1SetupResponseProtocolIEs(
        pub SequenceOf<AnonymousGNBCUUPE1SetupResponseProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " GNB-CU-UP E1 Setup Response"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GNB-CU-UP-E1SetupResponse")]
    #[non_exhaustive]
    pub struct GNBCUUPE1SetupResponse {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: GNBCUUPE1SetupResponseProtocolIEs,
    }
    impl GNBCUUPE1SetupResponse {
        pub fn new(protocol_ies: GNBCUUPE1SetupResponseProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousGNBCUUPStatusIndicationProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGNBCUUPStatusIndicationProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousGNBCUUPStatusIndicationProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousGNBCUUPStatusIndicationProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousGNBCUUPStatusIndicationProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct GNBCUUPStatusIndicationProtocolIEs(
        pub SequenceOf<AnonymousGNBCUUPStatusIndicationProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " gNB-CU-UP STATUS INDICATION ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " gNB-CU-UP Status Indication"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GNB-CU-UP-StatusIndication")]
    #[non_exhaustive]
    pub struct GNBCUUPStatusIndication {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: GNBCUUPStatusIndicationProtocolIEs,
    }
    impl GNBCUUPStatusIndication {
        pub fn new(protocol_ies: GNBCUUPStatusIndicationProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"), identifier = "GNB-CU-UP-TNLA-To-Remove-List")]
    pub struct GNBCUUPTNLAToRemoveList(pub SequenceOf<GNBCUUPTNLAToRemoveItem>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"), identifier = "IAB-Donor-CU-UPPSKInfo")]
    pub struct IABDonorCUUPPSKInfo(pub SequenceOf<IABDonorCUUPPSKInfoItem>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousIABUPTNLAddressUpdateProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousIABUPTNLAddressUpdateProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousIABUPTNLAddressUpdateProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousIABUPTNLAddressUpdateProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousIABUPTNLAddressUpdateProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct IABUPTNLAddressUpdateProtocolIEs(
        pub SequenceOf<AnonymousIABUPTNLAddressUpdateProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " IAB UP TNL ADDRESS UPDATE "]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " IAB UP TNL Address Update"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "IAB-UPTNLAddressUpdate")]
    #[non_exhaustive]
    pub struct IABUPTNLAddressUpdate {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: IABUPTNLAddressUpdateProtocolIEs,
    }
    impl IABUPTNLAddressUpdate {
        pub fn new(protocol_ies: IABUPTNLAddressUpdateProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousIABUPTNLAddressUpdateAcknowledgeProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousIABUPTNLAddressUpdateAcknowledgeProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousIABUPTNLAddressUpdateAcknowledgeProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousIABUPTNLAddressUpdateAcknowledgeProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousIABUPTNLAddressUpdateAcknowledgeProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct IABUPTNLAddressUpdateAcknowledgeProtocolIEs(
        pub SequenceOf<AnonymousIABUPTNLAddressUpdateAcknowledgeProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " IAB UP TNL Address Update Acknowledge"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "IAB-UPTNLAddressUpdateAcknowledge")]
    #[non_exhaustive]
    pub struct IABUPTNLAddressUpdateAcknowledge {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: IABUPTNLAddressUpdateAcknowledgeProtocolIEs,
    }
    impl IABUPTNLAddressUpdateAcknowledge {
        pub fn new(protocol_ies: IABUPTNLAddressUpdateAcknowledgeProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousIABUPTNLAddressUpdateFailureProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousIABUPTNLAddressUpdateFailureProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousIABUPTNLAddressUpdateFailureProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousIABUPTNLAddressUpdateFailureProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousIABUPTNLAddressUpdateFailureProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct IABUPTNLAddressUpdateFailureProtocolIEs(
        pub SequenceOf<AnonymousIABUPTNLAddressUpdateFailureProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " IAB UP TNL Address Update Failure"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "IAB-UPTNLAddressUpdateFailure")]
    #[non_exhaustive]
    pub struct IABUPTNLAddressUpdateFailure {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: IABUPTNLAddressUpdateFailureProtocolIEs,
    }
    impl IABUPTNLAddressUpdateFailure {
        pub fn new(protocol_ies: IABUPTNLAddressUpdateFailureProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousIABPSKNotificationProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousIABPSKNotificationProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousIABPSKNotificationProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousIABPSKNotificationProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousIABPSKNotificationProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct IABPSKNotificationProtocolIEs(
        pub SequenceOf<AnonymousIABPSKNotificationProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " IAB PSK NOTIFICATION"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " IAB PSK Notification"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct IABPSKNotification {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: IABPSKNotificationProtocolIEs,
    }
    impl IABPSKNotification {
        pub fn new(protocol_ies: IABPSKNotificationProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCBearerContextModificationConfirmProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCBearerContextModificationConfirmProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCBearerContextModificationConfirmProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousMCBearerContextModificationConfirmProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousMCBearerContextModificationConfirmProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct MCBearerContextModificationConfirmProtocolIEs(
        pub SequenceOf<AnonymousMCBearerContextModificationConfirmProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MC BEARER CONTEXT MODIFICATION CONFIRM"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCBearerContextModificationConfirm {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: MCBearerContextModificationConfirmProtocolIEs,
    }
    impl MCBearerContextModificationConfirm {
        pub fn new(protocol_ies: MCBearerContextModificationConfirmProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCBearerContextModificationFailureProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCBearerContextModificationFailureProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCBearerContextModificationFailureProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousMCBearerContextModificationFailureProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousMCBearerContextModificationFailureProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct MCBearerContextModificationFailureProtocolIEs(
        pub SequenceOf<AnonymousMCBearerContextModificationFailureProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MC BEARER CONTEXT MODIFICATION FAILURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCBearerContextModificationFailure {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: MCBearerContextModificationFailureProtocolIEs,
    }
    impl MCBearerContextModificationFailure {
        pub fn new(protocol_ies: MCBearerContextModificationFailureProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCBearerContextModificationRequestProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCBearerContextModificationRequestProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCBearerContextModificationRequestProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousMCBearerContextModificationRequestProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousMCBearerContextModificationRequestProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct MCBearerContextModificationRequestProtocolIEs(
        pub SequenceOf<AnonymousMCBearerContextModificationRequestProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MC BEARER CONTEXT MODIFICATION"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MC BEARER CONTEXT MODIFICATION REQUEST"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCBearerContextModificationRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: MCBearerContextModificationRequestProtocolIEs,
    }
    impl MCBearerContextModificationRequest {
        pub fn new(protocol_ies: MCBearerContextModificationRequestProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCBearerContextModificationRequiredProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCBearerContextModificationRequiredProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCBearerContextModificationRequiredProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousMCBearerContextModificationRequiredProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousMCBearerContextModificationRequiredProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct MCBearerContextModificationRequiredProtocolIEs(
        pub SequenceOf<AnonymousMCBearerContextModificationRequiredProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MC BEARER CONTEXT MODIFICATION REQUIRED"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MC BEARER CONTEXT MODIFICATION REQUIRED"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCBearerContextModificationRequired {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: MCBearerContextModificationRequiredProtocolIEs,
    }
    impl MCBearerContextModificationRequired {
        pub fn new(protocol_ies: MCBearerContextModificationRequiredProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCBearerContextModificationResponseProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCBearerContextModificationResponseProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCBearerContextModificationResponseProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousMCBearerContextModificationResponseProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousMCBearerContextModificationResponseProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct MCBearerContextModificationResponseProtocolIEs(
        pub SequenceOf<AnonymousMCBearerContextModificationResponseProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MC BEARER CONTEXT MODIFICATION RESPONSE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCBearerContextModificationResponse {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: MCBearerContextModificationResponseProtocolIEs,
    }
    impl MCBearerContextModificationResponse {
        pub fn new(protocol_ies: MCBearerContextModificationResponseProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCBearerContextReleaseCommandProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCBearerContextReleaseCommandProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCBearerContextReleaseCommandProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousMCBearerContextReleaseCommandProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousMCBearerContextReleaseCommandProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct MCBearerContextReleaseCommandProtocolIEs(
        pub SequenceOf<AnonymousMCBearerContextReleaseCommandProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MC BEARER CONTEXT RELEASE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MC BEARER CONTEXT RELEASE COMMAND"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCBearerContextReleaseCommand {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: MCBearerContextReleaseCommandProtocolIEs,
    }
    impl MCBearerContextReleaseCommand {
        pub fn new(protocol_ies: MCBearerContextReleaseCommandProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCBearerContextReleaseCompleteProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCBearerContextReleaseCompleteProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCBearerContextReleaseCompleteProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousMCBearerContextReleaseCompleteProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousMCBearerContextReleaseCompleteProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct MCBearerContextReleaseCompleteProtocolIEs(
        pub SequenceOf<AnonymousMCBearerContextReleaseCompleteProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MC BEARER CONTEXT RELEASE COMPLETE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCBearerContextReleaseComplete {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: MCBearerContextReleaseCompleteProtocolIEs,
    }
    impl MCBearerContextReleaseComplete {
        pub fn new(protocol_ies: MCBearerContextReleaseCompleteProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCBearerContextReleaseRequestProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCBearerContextReleaseRequestProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCBearerContextReleaseRequestProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousMCBearerContextReleaseRequestProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousMCBearerContextReleaseRequestProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct MCBearerContextReleaseRequestProtocolIEs(
        pub SequenceOf<AnonymousMCBearerContextReleaseRequestProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MC BEARER CONTEXT RELEASE REQUEST"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MC BEARER CONTEXT RELEASE REQUEST"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCBearerContextReleaseRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: MCBearerContextReleaseRequestProtocolIEs,
    }
    impl MCBearerContextReleaseRequest {
        pub fn new(protocol_ies: MCBearerContextReleaseRequestProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCBearerContextSetupFailureProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCBearerContextSetupFailureProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCBearerContextSetupFailureProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousMCBearerContextSetupFailureProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousMCBearerContextSetupFailureProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct MCBearerContextSetupFailureProtocolIEs(
        pub SequenceOf<AnonymousMCBearerContextSetupFailureProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MC BEARER CONTEXT SETUP FAILURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCBearerContextSetupFailure {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: MCBearerContextSetupFailureProtocolIEs,
    }
    impl MCBearerContextSetupFailure {
        pub fn new(protocol_ies: MCBearerContextSetupFailureProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCBearerContextSetupRequestProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCBearerContextSetupRequestProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCBearerContextSetupRequestProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousMCBearerContextSetupRequestProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousMCBearerContextSetupRequestProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct MCBearerContextSetupRequestProtocolIEs(
        pub SequenceOf<AnonymousMCBearerContextSetupRequestProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MC BEARER CONTEXT SETUP"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MC BEARER CONTEXT SETUP REQUEST"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCBearerContextSetupRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: MCBearerContextSetupRequestProtocolIEs,
    }
    impl MCBearerContextSetupRequest {
        pub fn new(protocol_ies: MCBearerContextSetupRequestProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCBearerContextSetupResponseProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCBearerContextSetupResponseProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCBearerContextSetupResponseProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousMCBearerContextSetupResponseProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousMCBearerContextSetupResponseProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct MCBearerContextSetupResponseProtocolIEs(
        pub SequenceOf<AnonymousMCBearerContextSetupResponseProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MC BEARER CONTEXT SETUP RESPONSE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCBearerContextSetupResponse {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: MCBearerContextSetupResponseProtocolIEs,
    }
    impl MCBearerContextSetupResponse {
        pub fn new(protocol_ies: MCBearerContextSetupResponseProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMCBearerNotificationProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMCBearerNotificationProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMCBearerNotificationProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousMCBearerNotificationProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousMCBearerNotificationProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct MCBearerNotificationProtocolIEs(
        pub SequenceOf<AnonymousMCBearerNotificationProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MC BEARER NOTIFICATION"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MCBearerNotification {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: MCBearerNotificationProtocolIEs,
    }
    impl MCBearerNotification {
        pub fn new(protocol_ies: MCBearerNotificationProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AnonymousMRDCDataUsageReportProtocolIEsCriticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMRDCDataUsageReportProtocolIEs {
        #[rasn(value("0..=65535"))]
        pub id: u16,
        pub criticality: AnonymousMRDCDataUsageReportProtocolIEsCriticality,
        pub value: Any,
    }
    impl AnonymousMRDCDataUsageReportProtocolIEs {
        pub fn new(
            id: u16,
            criticality: AnonymousMRDCDataUsageReportProtocolIEsCriticality,
            value: Any,
        ) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct MRDCDataUsageReportProtocolIEs(
        pub SequenceOf<AnonymousMRDCDataUsageReportProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MR-DC DATA USAGE REPORT"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MRDC-DataUsageReport")]
    #[non_exhaustive]
    pub struct MRDCDataUsageReport {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: MRDCDataUsageReportProtocolIEs,
    }
    impl MRDCDataUsageReport {
        pub fn new(protocol_ies: MRDCDataUsageReportProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPrivateMessagePrivateIEs {
        pub id: PrivateIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousPrivateMessagePrivateIEs {
        pub fn new(id: PrivateIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct PrivateMessagePrivateIEs(pub SequenceOf<AnonymousPrivateMessagePrivateIEs>);
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " PRIVATE MESSAGE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PrivateMessage {
        #[rasn(identifier = "privateIEs")]
        pub private_ies: PrivateMessagePrivateIEs,
    }
    impl PrivateMessage {
        pub fn new(private_ies: PrivateMessagePrivateIEs) -> Self {
            Self { private_ies }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousResetProtocolIEs {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousResetProtocolIEs {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct ResetProtocolIEs(pub SequenceOf<AnonymousResetProtocolIEs>);
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " RESET "]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Reset"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct Reset {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ResetProtocolIEs,
    }
    impl Reset {
        pub fn new(protocol_ies: ResetProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousResetAcknowledgeProtocolIEs {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousResetAcknowledgeProtocolIEs {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct ResetAcknowledgeProtocolIEs(pub SequenceOf<AnonymousResetAcknowledgeProtocolIEs>);
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Reset Acknowledge"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ResetAcknowledge {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ResetAcknowledgeProtocolIEs,
    }
    impl ResetAcknowledge {
        pub fn new(protocol_ies: ResetAcknowledgeProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ResetAll {
        #[rasn(identifier = "reset-all")]
        reset_all = 0,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ResetTypeChoiceExtension {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl ResetTypeChoiceExtension {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum ResetType {
        #[rasn(identifier = "e1-Interface")]
        e1_Interface(ResetAll),
        #[rasn(identifier = "partOfE1-Interface")]
        partOfE1_Interface(UEAssociatedLogicalE1ConnectionListRes),
        #[rasn(identifier = "choice-extension")]
        choice_extension(ResetTypeChoiceExtension),
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousResourceStatusFailureProtocolIEs {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousResourceStatusFailureProtocolIEs {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct ResourceStatusFailureProtocolIEs(
        pub SequenceOf<AnonymousResourceStatusFailureProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " RESOURCE STATUS FAILURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ResourceStatusFailure {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ResourceStatusFailureProtocolIEs,
    }
    impl ResourceStatusFailure {
        pub fn new(protocol_ies: ResourceStatusFailureProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousResourceStatusRequestProtocolIEs {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousResourceStatusRequestProtocolIEs {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct ResourceStatusRequestProtocolIEs(
        pub SequenceOf<AnonymousResourceStatusRequestProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " RESOURCE STATUS REQUEST"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ResourceStatusRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ResourceStatusRequestProtocolIEs,
    }
    impl ResourceStatusRequest {
        pub fn new(protocol_ies: ResourceStatusRequestProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousResourceStatusResponseProtocolIEs {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousResourceStatusResponseProtocolIEs {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct ResourceStatusResponseProtocolIEs(
        pub SequenceOf<AnonymousResourceStatusResponseProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " RESOURCE STATUS RESPONSE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ResourceStatusResponse {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ResourceStatusResponseProtocolIEs,
    }
    impl ResourceStatusResponse {
        pub fn new(protocol_ies: ResourceStatusResponseProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousResourceStatusUpdateProtocolIEs {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousResourceStatusUpdateProtocolIEs {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct ResourceStatusUpdateProtocolIEs(
        pub SequenceOf<AnonymousResourceStatusUpdateProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " RESOURCE STATUS UPDATE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ResourceStatusUpdate {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ResourceStatusUpdateProtocolIEs,
    }
    impl ResourceStatusUpdate {
        pub fn new(protocol_ies: ResourceStatusUpdateProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSupportedPLMNsItemIEExtensions {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousSupportedPLMNsItemIEExtensions {
        pub fn new(id: ProtocolIEID, criticality: Criticality, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct SupportedPLMNsItemIEExtensions(
        pub SequenceOf<AnonymousSupportedPLMNsItemIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SupportedPLMNs-Item")]
    #[non_exhaustive]
    pub struct SupportedPLMNsItem {
        #[rasn(identifier = "pLMN-Identity")]
        pub p_lmn_identity: PLMNIdentity,
        #[rasn(identifier = "slice-Support-List")]
        pub slice_support_list: Option<SliceSupportList>,
        #[rasn(identifier = "nR-CGI-Support-List")]
        pub n_r_cgi_support_list: Option<NRCGISupportList>,
        #[rasn(identifier = "qoS-Parameters-Support-List")]
        pub qo_s_parameters_support_list: Option<QoSParametersSupportList>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<SupportedPLMNsItemIEExtensions>,
    }
    impl SupportedPLMNsItem {
        pub fn new(
            p_lmn_identity: PLMNIdentity,
            slice_support_list: Option<SliceSupportList>,
            n_r_cgi_support_list: Option<NRCGISupportList>,
            qo_s_parameters_support_list: Option<QoSParametersSupportList>,
            i_e_extensions: Option<SupportedPLMNsItemIEExtensions>,
        ) -> Self {
            Self {
                p_lmn_identity,
                slice_support_list,
                n_r_cgi_support_list,
                qo_s_parameters_support_list,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=12"), identifier = "SupportedPLMNs-List")]
    pub struct SupportedPLMNsList(pub SequenceOf<SupportedPLMNsItem>);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSystemBearerContextModificationConfirmEUTRANBearerContextModificationConfirm {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousSystemBearerContextModificationConfirmEUTRANBearerContextModificationConfirm {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct SystemBearerContextModificationConfirmEUTRANBearerContextModificationConfirm(
        pub  SequenceOf<
            AnonymousSystemBearerContextModificationConfirmEUTRANBearerContextModificationConfirm,
        >,
    );
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSystemBearerContextModificationConfirmNGRANBearerContextModificationConfirm {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousSystemBearerContextModificationConfirmNGRANBearerContextModificationConfirm {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct SystemBearerContextModificationConfirmNGRANBearerContextModificationConfirm(
        pub  SequenceOf<
            AnonymousSystemBearerContextModificationConfirmNGRANBearerContextModificationConfirm,
        >,
    );
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SystemBearerContextModificationConfirmChoiceExtension {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl SystemBearerContextModificationConfirmChoiceExtension {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        choice,
        automatic_tags,
        identifier = "System-BearerContextModificationConfirm"
    )]
    pub enum SystemBearerContextModificationConfirm {
        #[rasn(identifier = "e-UTRAN-BearerContextModificationConfirm")]
        e_UTRAN_BearerContextModificationConfirm(
            SystemBearerContextModificationConfirmEUTRANBearerContextModificationConfirm,
        ),
        #[rasn(identifier = "nG-RAN-BearerContextModificationConfirm")]
        nG_RAN_BearerContextModificationConfirm(
            SystemBearerContextModificationConfirmNGRANBearerContextModificationConfirm,
        ),
        #[rasn(identifier = "choice-extension")]
        choice_extension(SystemBearerContextModificationConfirmChoiceExtension),
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSystemBearerContextModificationRequestEUTRANBearerContextModificationRequest {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousSystemBearerContextModificationRequestEUTRANBearerContextModificationRequest {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct SystemBearerContextModificationRequestEUTRANBearerContextModificationRequest(
        pub  SequenceOf<
            AnonymousSystemBearerContextModificationRequestEUTRANBearerContextModificationRequest,
        >,
    );
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSystemBearerContextModificationRequestNGRANBearerContextModificationRequest {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousSystemBearerContextModificationRequestNGRANBearerContextModificationRequest {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct SystemBearerContextModificationRequestNGRANBearerContextModificationRequest(
        pub  SequenceOf<
            AnonymousSystemBearerContextModificationRequestNGRANBearerContextModificationRequest,
        >,
    );
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SystemBearerContextModificationRequestChoiceExtension {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl SystemBearerContextModificationRequestChoiceExtension {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        choice,
        automatic_tags,
        identifier = "System-BearerContextModificationRequest"
    )]
    pub enum SystemBearerContextModificationRequest {
        #[rasn(identifier = "e-UTRAN-BearerContextModificationRequest")]
        e_UTRAN_BearerContextModificationRequest(
            SystemBearerContextModificationRequestEUTRANBearerContextModificationRequest,
        ),
        #[rasn(identifier = "nG-RAN-BearerContextModificationRequest")]
        nG_RAN_BearerContextModificationRequest(
            SystemBearerContextModificationRequestNGRANBearerContextModificationRequest,
        ),
        #[rasn(identifier = "choice-extension")]
        choice_extension(SystemBearerContextModificationRequestChoiceExtension),
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSystemBearerContextModificationRequiredEUTRANBearerContextModificationRequired {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousSystemBearerContextModificationRequiredEUTRANBearerContextModificationRequired {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct SystemBearerContextModificationRequiredEUTRANBearerContextModificationRequired(
        pub  SequenceOf<
            AnonymousSystemBearerContextModificationRequiredEUTRANBearerContextModificationRequired,
        >,
    );
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSystemBearerContextModificationRequiredNGRANBearerContextModificationRequired {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousSystemBearerContextModificationRequiredNGRANBearerContextModificationRequired {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct SystemBearerContextModificationRequiredNGRANBearerContextModificationRequired(
        pub  SequenceOf<
            AnonymousSystemBearerContextModificationRequiredNGRANBearerContextModificationRequired,
        >,
    );
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SystemBearerContextModificationRequiredChoiceExtension {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl SystemBearerContextModificationRequiredChoiceExtension {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        choice,
        automatic_tags,
        identifier = "System-BearerContextModificationRequired"
    )]
    pub enum SystemBearerContextModificationRequired {
        #[rasn(identifier = "e-UTRAN-BearerContextModificationRequired")]
        e_UTRAN_BearerContextModificationRequired(
            SystemBearerContextModificationRequiredEUTRANBearerContextModificationRequired,
        ),
        #[rasn(identifier = "nG-RAN-BearerContextModificationRequired")]
        nG_RAN_BearerContextModificationRequired(
            SystemBearerContextModificationRequiredNGRANBearerContextModificationRequired,
        ),
        #[rasn(identifier = "choice-extension")]
        choice_extension(SystemBearerContextModificationRequiredChoiceExtension),
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSystemBearerContextModificationResponseEUTRANBearerContextModificationResponse {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousSystemBearerContextModificationResponseEUTRANBearerContextModificationResponse {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct SystemBearerContextModificationResponseEUTRANBearerContextModificationResponse(
        pub  SequenceOf<
            AnonymousSystemBearerContextModificationResponseEUTRANBearerContextModificationResponse,
        >,
    );
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSystemBearerContextModificationResponseNGRANBearerContextModificationResponse {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousSystemBearerContextModificationResponseNGRANBearerContextModificationResponse {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct SystemBearerContextModificationResponseNGRANBearerContextModificationResponse(
        pub  SequenceOf<
            AnonymousSystemBearerContextModificationResponseNGRANBearerContextModificationResponse,
        >,
    );
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SystemBearerContextModificationResponseChoiceExtension {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl SystemBearerContextModificationResponseChoiceExtension {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        choice,
        automatic_tags,
        identifier = "System-BearerContextModificationResponse"
    )]
    pub enum SystemBearerContextModificationResponse {
        #[rasn(identifier = "e-UTRAN-BearerContextModificationResponse")]
        e_UTRAN_BearerContextModificationResponse(
            SystemBearerContextModificationResponseEUTRANBearerContextModificationResponse,
        ),
        #[rasn(identifier = "nG-RAN-BearerContextModificationResponse")]
        nG_RAN_BearerContextModificationResponse(
            SystemBearerContextModificationResponseNGRANBearerContextModificationResponse,
        ),
        #[rasn(identifier = "choice-extension")]
        choice_extension(SystemBearerContextModificationResponseChoiceExtension),
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSystemBearerContextSetupRequestEUTRANBearerContextSetupRequest {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousSystemBearerContextSetupRequestEUTRANBearerContextSetupRequest {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct SystemBearerContextSetupRequestEUTRANBearerContextSetupRequest(
        pub SequenceOf<AnonymousSystemBearerContextSetupRequestEUTRANBearerContextSetupRequest>,
    );
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSystemBearerContextSetupRequestNGRANBearerContextSetupRequest {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousSystemBearerContextSetupRequestNGRANBearerContextSetupRequest {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct SystemBearerContextSetupRequestNGRANBearerContextSetupRequest(
        pub SequenceOf<AnonymousSystemBearerContextSetupRequestNGRANBearerContextSetupRequest>,
    );
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SystemBearerContextSetupRequestChoiceExtension {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl SystemBearerContextSetupRequestChoiceExtension {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        choice,
        automatic_tags,
        identifier = "System-BearerContextSetupRequest"
    )]
    pub enum SystemBearerContextSetupRequest {
        #[rasn(identifier = "e-UTRAN-BearerContextSetupRequest")]
        e_UTRAN_BearerContextSetupRequest(
            SystemBearerContextSetupRequestEUTRANBearerContextSetupRequest,
        ),
        #[rasn(identifier = "nG-RAN-BearerContextSetupRequest")]
        nG_RAN_BearerContextSetupRequest(
            SystemBearerContextSetupRequestNGRANBearerContextSetupRequest,
        ),
        #[rasn(identifier = "choice-extension")]
        choice_extension(SystemBearerContextSetupRequestChoiceExtension),
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSystemBearerContextSetupResponseEUTRANBearerContextSetupResponse {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousSystemBearerContextSetupResponseEUTRANBearerContextSetupResponse {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct SystemBearerContextSetupResponseEUTRANBearerContextSetupResponse(
        pub SequenceOf<AnonymousSystemBearerContextSetupResponseEUTRANBearerContextSetupResponse>,
    );
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSystemBearerContextSetupResponseNGRANBearerContextSetupResponse {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousSystemBearerContextSetupResponseNGRANBearerContextSetupResponse {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct SystemBearerContextSetupResponseNGRANBearerContextSetupResponse(
        pub SequenceOf<AnonymousSystemBearerContextSetupResponseNGRANBearerContextSetupResponse>,
    );
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SystemBearerContextSetupResponseChoiceExtension {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl SystemBearerContextSetupResponseChoiceExtension {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        choice,
        automatic_tags,
        identifier = "System-BearerContextSetupResponse"
    )]
    pub enum SystemBearerContextSetupResponse {
        #[rasn(identifier = "e-UTRAN-BearerContextSetupResponse")]
        e_UTRAN_BearerContextSetupResponse(
            SystemBearerContextSetupResponseEUTRANBearerContextSetupResponse,
        ),
        #[rasn(identifier = "nG-RAN-BearerContextSetupResponse")]
        nG_RAN_BearerContextSetupResponse(
            SystemBearerContextSetupResponseNGRANBearerContextSetupResponse,
        ),
        #[rasn(identifier = "choice-extension")]
        choice_extension(SystemBearerContextSetupResponseChoiceExtension),
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSystemGNBCUUPCounterCheckRequestEUTRANGNBCUUPCounterCheckRequest {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousSystemGNBCUUPCounterCheckRequestEUTRANGNBCUUPCounterCheckRequest {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct SystemGNBCUUPCounterCheckRequestEUTRANGNBCUUPCounterCheckRequest(
        pub SequenceOf<AnonymousSystemGNBCUUPCounterCheckRequestEUTRANGNBCUUPCounterCheckRequest>,
    );
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSystemGNBCUUPCounterCheckRequestNGRANGNBCUUPCounterCheckRequest {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousSystemGNBCUUPCounterCheckRequestNGRANGNBCUUPCounterCheckRequest {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct SystemGNBCUUPCounterCheckRequestNGRANGNBCUUPCounterCheckRequest(
        pub SequenceOf<AnonymousSystemGNBCUUPCounterCheckRequestNGRANGNBCUUPCounterCheckRequest>,
    );
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SystemGNBCUUPCounterCheckRequestChoiceExtension {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl SystemGNBCUUPCounterCheckRequestChoiceExtension {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        choice,
        automatic_tags,
        identifier = "System-GNB-CU-UP-CounterCheckRequest"
    )]
    pub enum SystemGNBCUUPCounterCheckRequest {
        #[rasn(identifier = "e-UTRAN-GNB-CU-UP-CounterCheckRequest")]
        e_UTRAN_GNB_CU_UP_CounterCheckRequest(
            SystemGNBCUUPCounterCheckRequestEUTRANGNBCUUPCounterCheckRequest,
        ),
        #[rasn(identifier = "nG-RAN-GNB-CU-UP-CounterCheckRequest")]
        nG_RAN_GNB_CU_UP_CounterCheckRequest(
            SystemGNBCUUPCounterCheckRequestNGRANGNBCUUPCounterCheckRequest,
        ),
        #[rasn(identifier = "choice-extension")]
        choice_extension(SystemGNBCUUPCounterCheckRequestChoiceExtension),
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousTraceStartProtocolIEs {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousTraceStartProtocolIEs {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct TraceStartProtocolIEs(pub SequenceOf<AnonymousTraceStartProtocolIEs>);
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " TRACE ELEMENTARY PROCEDURES"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " TRACE START"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct TraceStart {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: TraceStartProtocolIEs,
    }
    impl TraceStart {
        pub fn new(protocol_ies: TraceStartProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousUEAssociatedLogicalE1ConnectionListRes {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousUEAssociatedLogicalE1ConnectionListRes {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=65536"),
        identifier = "UE-associatedLogicalE1-ConnectionListRes"
    )]
    pub struct UEAssociatedLogicalE1ConnectionListRes(
        pub SequenceOf<AnonymousUEAssociatedLogicalE1ConnectionListRes>,
    );
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousUEAssociatedLogicalE1ConnectionListResAck {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousUEAssociatedLogicalE1ConnectionListResAck {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=65536"),
        identifier = "UE-associatedLogicalE1-ConnectionListResAck"
    )]
    pub struct UEAssociatedLogicalE1ConnectionListResAck(
        pub SequenceOf<AnonymousUEAssociatedLogicalE1ConnectionListResAck>,
    );
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousULDataNotificationProtocolIEs {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousULDataNotificationProtocolIEs {
        pub fn new(id: ProtocolIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"))]
    pub struct ULDataNotificationProtocolIEs(
        pub SequenceOf<AnonymousULDataNotificationProtocolIEs>,
    );
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UL DATA NOTIFICATION"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ULDataNotification {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ULDataNotificationProtocolIEs,
    }
    impl ULDataNotification {
        pub fn new(protocol_ies: ULDataNotificationProtocolIEs) -> Self {
            Self { protocol_ies }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8"))]
    pub struct ULUPTNLAddressToUpdateList(pub SequenceOf<ULUPTNLAddressToUpdateItem>);
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod e1_ap_pdu_descriptions {
    extern crate alloc;
    use super::e1_ap_common_data_types::{Criticality, ProcedureCode};
    use super::e1_ap_constants::{
        ID_BCBEARER_CONTEXT_MODIFICATION, ID_BCBEARER_CONTEXT_MODIFICATION_REQUIRED,
        ID_BCBEARER_CONTEXT_RELEASE, ID_BCBEARER_CONTEXT_RELEASE_REQUEST,
        ID_BCBEARER_CONTEXT_SETUP, ID_BEARER_CONTEXT_INACTIVITY_NOTIFICATION,
        ID_BEARER_CONTEXT_MODIFICATION, ID_BEARER_CONTEXT_MODIFICATION_REQUIRED,
        ID_BEARER_CONTEXT_RELEASE, ID_BEARER_CONTEXT_RELEASE_REQUEST, ID_BEARER_CONTEXT_SETUP,
        ID_CELL_TRAFFIC_TRACE, ID_DATA_USAGE_REPORT, ID_DEACTIVATE_TRACE, ID_D_LDATA_NOTIFICATION,
        ID_E1_RELEASE, ID_EARLY_FORWARDING_SNTRANSFER, ID_ERROR_INDICATION,
        ID_G_NB_CU_CPMEASUREMENT_RESULTS_INFORMATION, ID_G_NB_CU_CP_CONFIGURATION_UPDATE,
        ID_G_NB_CU_CP_E1_SETUP, ID_G_NB_CU_UP_CONFIGURATION_UPDATE, ID_G_NB_CU_UP_COUNTER_CHECK,
        ID_G_NB_CU_UP_E1_SETUP, ID_G_NB_CU_UP_STATUS_INDICATION, ID_I_ABPSKNOTIFICATION,
        ID_I_AB_UPTNLADDRESS_UPDATE, ID_MCBEARER_CONTEXT_MODIFICATION,
        ID_MCBEARER_CONTEXT_MODIFICATION_REQUIRED, ID_MCBEARER_CONTEXT_RELEASE,
        ID_MCBEARER_CONTEXT_RELEASE_REQUEST, ID_MCBEARER_CONTEXT_SETUP, ID_MCBEARER_NOTIFICATION,
        ID_M_RDC_DATA_USAGE_REPORT, ID_PRIVATE_MESSAGE, ID_RESET, ID_RESOURCE_STATUS_REPORTING,
        ID_RESOURCE_STATUS_REPORTING_INITIATION, ID_TRACE_START, ID_U_LDATA_NOTIFICATION,
    };
    use super::e1_ap_pdu_contents::{
        BCBearerContextModificationConfirm, BCBearerContextModificationFailure,
        BCBearerContextModificationRequest, BCBearerContextModificationRequired,
        BCBearerContextModificationResponse, BCBearerContextReleaseCommand,
        BCBearerContextReleaseComplete, BCBearerContextReleaseRequest, BCBearerContextSetupFailure,
        BCBearerContextSetupRequest, BCBearerContextSetupResponse,
        BearerContextInactivityNotification, BearerContextModificationConfirm,
        BearerContextModificationFailure, BearerContextModificationRequest,
        BearerContextModificationRequired, BearerContextModificationResponse,
        BearerContextReleaseCommand, BearerContextReleaseComplete, BearerContextReleaseRequest,
        BearerContextSetupFailure, BearerContextSetupRequest, BearerContextSetupResponse,
        CellTrafficTrace, DLDataNotification, DataUsageReport, DeactivateTrace, E1ReleaseRequest,
        E1ReleaseResponse, EarlyForwardingSNTransfer, ErrorIndication, GNBCUCPConfigurationUpdate,
        GNBCUCPConfigurationUpdateAcknowledge, GNBCUCPConfigurationUpdateFailure,
        GNBCUCPE1SetupFailure, GNBCUCPE1SetupRequest, GNBCUCPE1SetupResponse,
        GNBCUCPMeasurementResultsInformation, GNBCUUPConfigurationUpdate,
        GNBCUUPConfigurationUpdateAcknowledge, GNBCUUPConfigurationUpdateFailure,
        GNBCUUPCounterCheckRequest, GNBCUUPE1SetupFailure, GNBCUUPE1SetupRequest,
        GNBCUUPE1SetupResponse, GNBCUUPStatusIndication, IABPSKNotification, IABUPTNLAddressUpdate,
        IABUPTNLAddressUpdateAcknowledge, IABUPTNLAddressUpdateFailure,
        MCBearerContextModificationConfirm, MCBearerContextModificationFailure,
        MCBearerContextModificationRequest, MCBearerContextModificationRequired,
        MCBearerContextModificationResponse, MCBearerContextReleaseCommand,
        MCBearerContextReleaseComplete, MCBearerContextReleaseRequest, MCBearerContextSetupFailure,
        MCBearerContextSetupRequest, MCBearerContextSetupResponse, MCBearerNotification,
        MRDCDataUsageReport, PrivateMessage, Reset, ResetAcknowledge, ResourceStatusFailure,
        ResourceStatusRequest, ResourceStatusResponse, ResourceStatusUpdate, TraceStart,
        ULDataNotification,
    };
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Interface Elementary Procedure Class"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    pub trait E1APELEMENTARYPROCEDURE {
        type InitiatingMessage: rasn::prelude::AsnType;
        type SuccessfulOutcome: rasn::prelude::AsnType;
        type UnsuccessfulOutcome: rasn::prelude::AsnType;
        const PROCEDURE_CODE: ProcedureCode;
        const CRITICALITY: Criticality;
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Interface PDU Definition"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags, identifier = "E1AP-PDU")]
    #[non_exhaustive]
    pub enum E1APPDU {
        initiatingMessage(InitiatingMessage),
        successfulOutcome(SuccessfulOutcome),
        unsuccessfulOutcome(UnsuccessfulOutcome),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct InitiatingMessage {
        #[rasn(identifier = "procedureCode")]
        pub procedure_code: ProcedureCode,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl InitiatingMessage {
        pub fn new(procedure_code: ProcedureCode, criticality: Criticality, value: Any) -> Self {
            Self {
                procedure_code,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SuccessfulOutcome {
        #[rasn(identifier = "procedureCode")]
        pub procedure_code: ProcedureCode,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl SuccessfulOutcome {
        pub fn new(procedure_code: ProcedureCode, criticality: Criticality, value: Any) -> Self {
            Self {
                procedure_code,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct UnsuccessfulOutcome {
        #[rasn(identifier = "procedureCode")]
        pub procedure_code: ProcedureCode,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl UnsuccessfulOutcome {
        pub fn new(procedure_code: ProcedureCode, criticality: Criticality, value: Any) -> Self {
            Self {
                procedure_code,
                criticality,
                value,
            }
        }
    }
}
