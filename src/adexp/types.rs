use serde::{Deserialize, Serialize};
use crate::adexp::error::AdexpError;

/// Types de messages ADEXP selon la spécification 3.4
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageType {
    /// Flight Plan (FPL)
    FlightPlan,
    
    /// Change (CHG)
    Change,
    
    /// Delay (DLA)
    Delay,
    
    /// Cancel (CNL)
    Cancel,
    
    /// Departure (DEP)
    Departure,
    
    /// Arrival (ARR)
    Arrival,
    
    /// Coordination (COF)
    Coordination,
    
    /// Request (REQ)
    Request,
    
    /// Estimate (EST)
    Estimate,
    
    /// Position (POS)
    Position,
    
    /// Logon (LOG)
    Logon,
    
    /// Logoff (LOF)
    Logoff,
    
    // Standard message titles - ADEXP 3.4
    /// Advance Boundary Information Message (ABI)
    Abi,
    /// Acknowledge Message (ACK)
    Ack,
    /// Acceptance Message (ACP)
    Acp,
    /// Activation Message (ACT)
    Act,
    /// Arrival Management Message (AMA)
    Ama,
    /// Aircraft Position Report Message (APR)
    Apr,
    /// Airspace Use Plan Message (AUP)
    Aup,
    /// Basic Flight Data Message (BFD)
    Bfd,
    /// Code Assignment Message (CAM)
    Cam,
    /// Code Cancellation Message (CCM)
    Ccm,
    /// Co-ordination Message (CDN)
    Cdn,
    /// Change to Flight Data Message (CFD)
    Cfd,
    /// SSR Code Assignment Message (COD)
    Cod,
    /// Code Request Message (COR)
    Cor,
    /// Conditional Route Availability Message (CRAM)
    Cram,
    /// Code Release Message (CRE)
    Cre,
    /// Clearance Response Message (CRP)
    Crp,
    /// Clearance Request Message (CRQ)
    Crq,
    /// De-Suspension Message (DES)
    Des,
    /// Departure Planning Information Message (DPI)
    Dpi,
    /// ETFMS Flight Data Message (EFD)
    Efd,
    /// Error Message (ERR)
    Err,
    /// Flight Confirmation Message (FCM)
    Fcm,
    /// Flight Suspension Message (FLS)
    Fls,
    /// First System Activation Message (FSA)
    Fsa,
    /// Flight Update Message (FUM)
    Fum,
    /// Hand-Over Proposal Message (HOP)
    Hop,
    /// Individual ATC Modification Message (IACH)
    Iach,
    /// Individual ATC Flight Plan Proposal Message (IAFP)
    Iafp,
    /// Individual ATC Flight Plan Message (IAPL)
    Iapl,
    /// Individual Arrival Message (IARR)
    Iarr,
    /// Individual Modification Message (ICHG)
    Ichg,
    /// Individual Cancellation Message (ICNL)
    Icnl,
    /// Individual Departure Message (IDEP)
    Idep,
    /// Individual Delay Message (IDLA)
    Idla,
    /// Individual Flight Plan Message (IFPL)
    Ifpl,
    /// Information Message (INF)
    Inf,
    /// Individual Repetitive Flight Plan (IRPL)
    Irpl,
    /// Individual Request Flight Plan Message (IRQP)
    Irqp,
    /// Logical Acknowledgement Message (LAM)
    Lam,
    /// Logical Rejection Message (LRM)
    Lrm,
    /// Message for Abrogation of Co-ordination (MAC)
    Mac,
    /// Manual Processing Pending Message (MAN)
    Man,
    /// Manual Assumption of Communications Message (MAS)
    Mas,
    /// Next Authority Notified Message (NAN)
    Nan,
    /// Oceanic Clearance Message (OCM)
    Ocm,
    /// Preliminary Activation Message (PAC)
    Pac,
    /// Point Message (PNT)
    Pnt,
    /// Referred Activate Proposal Message (RAP)
    Rap,
    /// Repetitive Flight Plan Data Modification Message (RCHG)
    Rchg,
    /// Request Oceanic Clearance Message (RCL)
    Rcl,
    /// Repetitive Flight Plan Data Cancellation Message (RCNL)
    Rcnl,
    /// Ready Message (REA)
    Rea,
    /// Rejection Message (REJ)
    Rej,
    /// Revision Message (REV)
    Rev,
    /// Ready for Improvement (RFI)
    Rfi,
    /// Reject Co-ordination Message (RJC)
    Rjc,
    /// Re-Routing Rejection Message (RJT)
    Rjt,
    /// Release Message (RLS)
    Rls,
    /// Request On Frequency Message (ROF)
    Rof,
    /// Re-Routing Notification (RRN)
    Rrn,
    /// Release Request Message (RRQ)
    Rrq,
    /// Re-Routing Proposal Message (RRP)
    Rrp,
    /// Referred Revision Proposal Message (RRV)
    Rrv,
    /// Request Tactical Instructions Message (RTI)
    Rti,
    /// Slot Allocation Message (SAM)
    Sam,
    /// Stand-by Message (SBY)
    Sby,
    /// Skip Communication (SCO)
    Sco,
    /// Supplementary Data Message (SDM)
    Sdm,
    /// Slot Improvement Proposal Message (SIP)
    Sip,
    /// Skip Cancellation Message (SKC)
    Skc,
    /// Slot Requirement Cancellation Message (SLC)
    Slc,
    /// Slot Missed Message (SMM)
    Smm,
    /// Slot Proposal Acceptance Message (SPA)
    Spa,
    /// Slot Proposal Rejection Message (SRJ)
    Srj,
    /// Slot Revision Message (SRM)
    Srm,
    /// SIP Wanted Message (SWM)
    Swm,
    /// Transfer Initiation Message (TIM)
    Tim,
    /// Tactical Instructions Proposal Message (TIP)
    Tip,
    /// Updated Airspace Use Plan Message (UUP)
    Uup,
    /// Warning Message (WAR)
    War,
    /// Crossing Alternate Proposal Message (XAP)
    Xap,
    /// Crossing Cancellation Message (XCM)
    Xcm,
    /// Crossing Intention Notification Message (XIN)
    Xin,
    /// Crossing Request Message (XRQ)
    Xrq,
    
    // Reserved message titles - Generated from ADEXP 3.4 Annex C
    /// Activation Message for a Departure (ACTDEP) - Reserved by FRANCE
    Actdep,

    /// ATFM Notification Message (ANM) - Reserved by NM
    Anm,

    /// Response Message Terminal Control System (ANSWERCT) - Reserved by FRANCE
    Answerct,

    /// Response Message ODS (ANSWM) - Reserved by FRANCE
    Answm,

    /// Response Message (ANSXFPLCT) - Reserved by FRANCE
    Ansxfplct,

    /// Landing Message (ATT) - Reserved by FRANCE
    Att,

    /// Begin Processing Message (BEGINPROC) - Reserved by FRANCE
    Beginproc,

    /// Controller Working Position Initialisation Procedure Message ODS (BEGPROC) - Reserved by FRANCE
    Begproc,

    /// Controller Working Position Initialisation Message Terminal Control System (BEGPROCCT) - Reserved by FRANCE
    Begprocct,

    /// Departure Cleanance Message ARINC 623 (CDA) - Reserved by FRANCE
    Cda,

    /// Departure Clearance ARINC 620 (CDAFTX) - Reserved by FRANCE
    Cdaftx,

    /// Modification message for a Departure flight (CHGDEP) - Reserved by FRANCE
    Chgdep,

    /// Change to Flight Plan Data Message (CHGMSG) - Reserved by GERMANY
    Chgmsg,

    /// Departure Clearance ARINC 623 (CLD) - Reserved by FRANCE
    Cld,

    /// Departure Clearance ARINC620 (CLDFTX) - Reserved by FRANCE
    Cldftx,

    /// Cancellation of an Arrival (CNLARR) - Reserved by FRANCE
    Cnlarr,

    /// Cancellation of Exceptional Condition (CNLCOND) - Reserved by NM
    Cnlcond,

    /// Cancellation of a Departure (CNLDEP) - Reserved by FRANCE
    Cnldep,

    /// Cancellation of Flight Plan Data Message (CNLMSG) - Reserved by GERMANY
    Cnlmsg,

    /// Cancellation of an ATFM Regulation (CNLREG) - Reserved by NM
    Cnlreg,

    /// End Message to a change of Operational Configuration (CONFEND) - Reserved by FRANCE
    Confend,

    /// Operational Configuration Message ODS (CONFIDM) - Reserved by FRANCE
    Confidm,

    /// Operational Configuration Message Terminal Control System (CONFIDMCT) - Reserved by FRANCE
    Confidmct,

    /// Activation Message for an Arrival (CTARR) - Reserved by FRANCE
    Ctarr,

    /// Take-Off Message (DEC) - Reserved by FRANCE
    Dec,

    /// Duplication Flight Plan Message (DOUBM) - Reserved by FRANCE
    Doubm,

    /// Modification of Destination Message (DRT) - Reserved by FRANCE
    Drt,

    /// Update of Estimated Arrival Time Message (EATARR) - Reserved by FRANCE
    Eatarr,

    /// Controller Working Position Initialisation Procedure Last Message ODS (ENDPROC) - Reserved by FRANCE
    Endproc,

    /// Controller Working Position Initialisation Procedure Last Message Terminal Control System (ENDPROCCT) - Reserved by FRANCE
    Endprocct,

    /// Event Message (EVENT) - Reserved by GERMANY
    Event,

    /// Pre-Activation Message for Arrival (EVLARR) - Reserved by FRANCE
    Evlarr,

    /// Pre-Activation Message for Departure (EVLDEP) - Reserved by FRANCE
    Evldep,

    /// Activation of an Exceptional Condition (EXCOND) - Reserved by NM
    Excond,

    /// Flight Data Creation Message (FICM) - Reserved by FRANCE
    Ficm,

    /// Flexible Track Description Display Message (FLXVIVO) - Reserved by FRANCE
    Flxvivo,

    /// Flight Plan Data Close Message ODS (FPCLOSE) - Reserved by FRANCE
    Fpclose,

    /// Flight Plan Data Close Message Terminal Control System (FPCLOSECT) - Reserved by FRANCE
    Fpclosect,

    /// Duplication of Flight Plan Data Close Message ODS (FPCLOSED) - Reserved by FRANCE
    Fpclosed,

    /// Activation of Flight Plan Message ODS (FPCRD) - Reserved by FRANCE
    Fpcrd,

    /// Activation of Flight Plan Message Terminal Control System (FPCRDCT) - Reserved by FRANCE
    Fpcrdct,

    /// Duplication of Flight Plan Data Activation Message ODS (FPCRDD) - Reserved by FRANCE
    Fpcrdd,

    /// Creation of Flight Plan Message ODS (FPCRE) - Reserved by FRANCE
    Fpcre,

    /// Creation of Flight Plan Message Terminal Control System (FPCRECT) - Reserved by FRANCE
    Fpcrect,

    /// Pre-Activation of Flight Plan Message ODS (FPINI) - Reserved by FRANCE
    Fpini,

    /// Pre-Activation of Flight Plan Message Terminal Control System (FPINICT) - Reserved by FRANCE
    Fpinict,

    /// Duplication of Pre-Activation of Flight Plan Message (FPINID) - Reserved by FRANCE
    Fpinid,

    /// Flight Plan Data Message (FPLMSG) - Reserved by GERMANY
    Fplmsg,

    /// Pre-Activation of Flight Plan Message ODS (FPNTF) - Reserved by FRANCE
    Fpntf,

    /// Duplication of Pre-Activation of Flight Plan Message ODS (FPNTFD) - Reserved by FRANCE
    Fpntfd,

    /// Flight Data Information Message for a Non-Concerned Sector ODS (FPRDU) - Reserved by FRANCE
    Fprdu,

    /// Flight Data Information Message for a non-concerned Sector Terminal Control System (FPRDUCT) - Reserved by FRANCE
    Fprduct,

    /// Departure Clearance System Message ARINC 623 (FSM) - Reserved by FRANCE
    Fsm,

    /// Departure Clearance System Message ARINC 620 (FSMFTX) - Reserved by FRANCE
    Fsmftx,

    /// Flight Suspension Request Message (FSR) - Reserved by NM
    Fsr,

    /// Individual Flight Plan Data Query Message (IFPDQ) - Reserved by NM
    Ifpdq,

    /// Individual Flight Plan Data Query Reply Message (IFPDQR) - Reserved by NM
    Ifpdqr,

    /// Individual Flight Plan Data Summary Query Message (IFPDSQ) - Reserved by NM
    Ifpdsq,

    /// Individual Flight Plan Data Summary Query Reply Message (IFPDSQR) - Reserved by NM
    Ifpdsqr,

    /// Information Message (INFOM) - Reserved by FRANCE
    Infom,

    /// Individual Request for Supplementary Information Message (IRQS) - Reserved by NM
    Irqs,

    /// Individual Supplementary Flight Plan Message (ISPL) - Reserved by NM
    Ispl,

    /// Flight Plan Message List (LGR) - Reserved by FRANCE
    Lgr,

    /// Flight Plan Message List ODS (LISTFP) - Reserved by FRANCE
    Listfp,

    /// Flight Plan Message List Terminal Control System (LISTFPCT) - Reserved by FRANCE
    Listfpct,

    /// Identification of Flight Plan Message (LOGON) - Reserved by FRANCE
    LogonReserved,

    /// Daily Movements Message (MAJVIVO) - Reserved by FRANCE
    Majvivo,

    /// Co-ordination Message (MCOM) - Reserved by FRANCE
    Mcom,

    /// Modification of an Exceptional Condition (MODCOND) - Reserved by NM
    Modcond,

    /// Modification of an ATFM Regulation (MODREG) - Reserved by NM
    Modreg,

    /// Activation of a Mandatory Route (MRA) - Reserved by NM
    Mra,

    /// Cancellation of a Mandatory Route (MRCNL) - Reserved by NM
    Mrcnl,

    /// Modification of a Mandatory Route (MRMOD) - Reserved by NM
    Mrmod,

    /// Mandatory Re-Routing Message (MRR) - Reserved by NM
    Mrr,

    /// Movements Information Message (MVTVIVO) - Reserved by FRANCE
    Mvtvivo,

    /// Activation of an ATFM Regulation (NEWREG) - Reserved by NM
    Newreg,

    /// Activation of a Not Allowed Traffic Flow (NTA) - Reserved by NM
    Nta,

    /// Cancellation of a Not Allowed Traffic Flow (NTACNL) - Reserved by NM
    Ntacnl,

    /// Modification of a Not Allowed Traffic Flow (NTAMOD) - Reserved by NM
    Ntamod,

    /// Oceanic Clearance Message (OCLM) - Reserved by FRANCE
    Oclm,

    /// Duplication of Oceanic Clearance Message (OCLMD) - Reserved by FRANCE
    Oclmd,

    /// Activation of an Off-Load Route (OLRA) - Reserved by NM
    Olra,

    /// Cancellation of an Off-Load Route (OLRCNL) - Reserved by NM
    Olrcnl,

    /// Modification of an Off-Load Route (OLRMOD) - Reserved by NM
    Olrmod,

    /// Runway Application Message (PAMAER) - Reserved by FRANCE
    Pamaer,

    /// On-Stand Confirmation Message (PAMARB) - Reserved by FRANCE
    Pamarb,

    /// Cancellation of Parking Allocation for an Arrival (PAMARRANN) - Reserved by FRANCE
    Pamarrann,

    /// Allocation of Parking Position for an Arrival (PAMARRCRE) - Reserved by FRANCE
    Pamarrcre,

    /// Modification of Parking Allocation for an Arrival (PAMARRPST) - Reserved by FRANCE
    Pamarrpst,

    /// Parking Message for Arrival Aircraft (PAMDAPARB) - Reserved by FRANCE
    Pamdaparb,

    /// Allocation of a Parking Position (PAMDAPCRE) - Reserved by FRANCE
    Pamdapcre,

    /// Cancellation of Parking Allocation for a Departure (PAMDEPANN) - Reserved by FRANCE
    Pamdepann,

    /// Parking Allocation for a Departure (PAMDEPCRE) - Reserved by FRANCE
    Pamdepcre,

    /// Modification of Parking Allocation for a Departure (PAMDEPPST) - Reserved by FRANCE
    Pamdeppst,

    /// Off-Stand Confirmation Message (PAMDRB) - Reserved by FRANCE
    Pamdrb,

    /// Return to Original Created Status for an Arrival (QTAARR) - Reserved by FRANCE
    Qtaarr,

    /// Return to Original Created Status for a Departure (QTADEP) - Reserved by FRANCE
    Qtadep,

    /// Request Departure Clearance Message ARINC 623 (RCD) - Reserved by FRANCE
    Rcd,

    /// Request Departure Clearance Message ARINC 620 (RCDFTX) - Reserved by FRANCE
    Rcdftx,

    /// Revision Message for an Arrival (REVARR) - Reserved by FRANCE
    Revarr,

    /// Repetitive Flight Plan Data Query Message (RFPDQ) - Reserved by NM
    Rfpdq,

    /// Repetitive Flight Plan Data Query Reply Message (RFPDQR) - Reserved by NM
    Rfpdqr,

    /// Repetitive Flight Plan Data Summary Query Message (RFPDSQ) - Reserved by NM
    Rfpdsq,

    /// Repetitive Flight Plan Data Summary Query Reply Message (RFPDSQR) - Reserved by NM
    Rfpdsqr,

    /// Flight Data Information Message (RIEM) - Reserved by FRANCE
    Riem,

    /// Missed Approach Message (RMG) - Reserved by FRANCE
    Rmg,

    /// Re-Routing Acceptance Message (RRA) - Reserved by NM
    Rra,

    /// Repetitive Flight Plan Recovery Message (RREC) - Reserved by NM
    Rrec,

    /// Re-Routing Notification Message (RRN) - Reserved by NM
    RrnReserved,

    /// Repetitive Flight Plan Suspension Message (RSUS) - Reserved by NM
    Rsus,

    /// Runway Configuration Message (RWYCHGCT) - Reserved by FRANCE
    Rwychgct,

    /// Runway Information Message (RWYMSG) - Reserved by GERMANY
    Rwymsg,

    /// Request for Flight Plan Activation ODS (TRACT) - Reserved by FRANCE
    Tract,

    /// Request for Flight Plan Activation Terminal Control System (TRACTCT) - Reserved by FRANCE
    Tractct,

    /// Request for Flight Plan Cancellation ODS (TRCNL) - Reserved by FRANCE
    Trcnl,

    /// Request for Flight Plan Cancellation Terminal Control System (TRCNLCT) - Reserved by FRANCE
    Trcnlct,

    /// Request for Manual Correlation (TRCOR) - Reserved by FRANCE
    Trcor,

    /// Request for Manual De-Correlation (TRDECOR) - Reserved by FRANCE
    Trdecor,

    /// Request for Creation of Flight Plan Data ODS (TRFIC) - Reserved by FRANCE
    Trfic,

    /// Request for Creation of Flight Plan Data Terminal Control System (TRFICCT) - Reserved by FRANCE
    Trficct,

    /// Request Flight Level Message (TRFLRQT) - Reserved by FRANCE
    Trflrqt,

    /// Request for Flight Plan Modification ODS (TRMOD) - Reserved by FRANCE
    Trmod,

    /// Request for Flight Plan Modification Terminal Control System (TRMODCT) - Reserved by FRANCE
    Trmodct,

    /// Request for Time Modification (TRMODH) - Reserved by FRANCE
    Trmodh,

    /// Request for Time Modification for Delayed Flight (TRMODHD) - Reserved by FRANCE
    Trmodhd,

    /// Co-ordination Request for Exiting Flight ODS (TRMVT) - Reserved by FRANCE
    Trmvt,

    /// Co-ordination Request for Exiting Flight Terminal Control System (TRMVTCT) - Reserved by FRANCE
    Trmvtct,

    /// Specific Flight Data Request Message (TRPOINT) - Reserved by FRANCE
    Trpoint,

    /// Request for Revision of Flight Plan to Created Status ODS (TRRET) - Reserved by FRANCE
    Trret,

    /// Request for Revision of Flight Plan to Created Status Terminal Control System (TRRETCT) - Reserved by FRANCE
    Trretct,

    /// Request for Display of Flight Data Information ODS (TRRIP) - Reserved by FRANCE
    Trrip,

    /// Request for Display of Flight Data Information Terminal Control System (TRRIPCT) - Reserved by FRANCE
    Trripct,

    /// Flight Plan Request ODS (TRRQT) - Reserved by FRANCE
    Trrqt,

    /// Flight Plan Request Terminal Control System (TRRQTCT) - Reserved by FRANCE
    Trrqtct,

    /// Request for SHOOT Action (TRSHRQT) - Reserved by FRANCE
    Trshrqt,

    /// Controller Working Position Initialisation Request ODS (TRSTAR) - Reserved by FRANCE
    Trstar,

    /// Controller Working Position Initialisation Request Terminal Control System (TRSTARCT) - Reserved by FRANCE
    Trstarct,

    /// Transfer Position Message (TRTRP) - Reserved by FRANCE
    Trtrp,

    /// Target Time message (TTIME) - Reserved by GERMANY
    Ttime,

    /// Suppression of Flight Plan Message ODS (UNKFP) - Reserved by FRANCE
    Unkfp,

    /// Suppression of Flight Plan Message Terminal Control System (UNKFPCT) - Reserved by FRANCE
    Unkfpct,

    /// Message générique non catégorisé
    Generic,
}

impl MessageType {
    /// Parse un type depuis le champ TITLE
    pub fn from_title(title: &str) -> Result<Self, AdexpError> {
        let title_upper = title.to_uppercase();
        match title_upper.as_str() {
            "FPL" => Ok(MessageType::FlightPlan),
            "CHG" => Ok(MessageType::Change),
            "DLA" => Ok(MessageType::Delay),
            "CNL" => Ok(MessageType::Cancel),
            "DEP" => Ok(MessageType::Departure),
            "ARR" => Ok(MessageType::Arrival),
            "COF" => Ok(MessageType::Coordination),
            "REQ" => Ok(MessageType::Request),
            "EST" => Ok(MessageType::Estimate),
            "POS" => Ok(MessageType::Position),
            "LOG" => Ok(MessageType::Logon),
            // Standard message titles
            "ABI" => Ok(MessageType::Abi),
            "ACK" => Ok(MessageType::Ack),
            "ACP" => Ok(MessageType::Acp),
            "ACT" => Ok(MessageType::Act),
            "AMA" => Ok(MessageType::Ama),
            "APR" => Ok(MessageType::Apr),
            "AUP" => Ok(MessageType::Aup),
            "BFD" => Ok(MessageType::Bfd),
            "CAM" => Ok(MessageType::Cam),
            "CCM" => Ok(MessageType::Ccm),
            "CDN" => Ok(MessageType::Cdn),
            "CFD" => Ok(MessageType::Cfd),
            "COD" => Ok(MessageType::Cod),
            "COR" => Ok(MessageType::Cor),
            "CRAM" => Ok(MessageType::Cram),
            "CRE" => Ok(MessageType::Cre),
            "CRP" => Ok(MessageType::Crp),
            "CRQ" => Ok(MessageType::Crq),
            "DES" => Ok(MessageType::Des),
            "DPI" => Ok(MessageType::Dpi),
            "EFD" => Ok(MessageType::Efd),
            "ERR" => Ok(MessageType::Err),
            "FCM" => Ok(MessageType::Fcm),
            "FLS" => Ok(MessageType::Fls),
            "FSA" => Ok(MessageType::Fsa),
            "FUM" => Ok(MessageType::Fum),
            "HOP" => Ok(MessageType::Hop),
            "IACH" => Ok(MessageType::Iach),
            "IAFP" => Ok(MessageType::Iafp),
            "IAPL" => Ok(MessageType::Iapl),
            "IARR" => Ok(MessageType::Iarr),
            "ICHG" => Ok(MessageType::Ichg),
            "ICNL" => Ok(MessageType::Icnl),
            "IDEP" => Ok(MessageType::Idep),
            "IDLA" => Ok(MessageType::Idla),
            "IFPL" => Ok(MessageType::Ifpl),
            "INF" => Ok(MessageType::Inf),
            "IRPL" => Ok(MessageType::Irpl),
            "IRQP" => Ok(MessageType::Irqp),
            "LAM" => Ok(MessageType::Lam),
            "LRM" => Ok(MessageType::Lrm),
            "MAC" => Ok(MessageType::Mac),
            "MAN" => Ok(MessageType::Man),
            "MAS" => Ok(MessageType::Mas),
            "NAN" => Ok(MessageType::Nan),
            "OCM" => Ok(MessageType::Ocm),
            "PAC" => Ok(MessageType::Pac),
            "PNT" => Ok(MessageType::Pnt),
            "RAP" => Ok(MessageType::Rap),
            "RCHG" => Ok(MessageType::Rchg),
            "RCL" => Ok(MessageType::Rcl),
            "RCNL" => Ok(MessageType::Rcnl),
            "REA" => Ok(MessageType::Rea),
            "REJ" => Ok(MessageType::Rej),
            "REV" => Ok(MessageType::Rev),
            "RFI" => Ok(MessageType::Rfi),
            "RJC" => Ok(MessageType::Rjc),
            "RJT" => Ok(MessageType::Rjt),
            "RLS" => Ok(MessageType::Rls),
            "ROF" => Ok(MessageType::Rof),
            "RRN" => Ok(MessageType::Rrn),  // Standard title takes priority
            // Note: RRN also exists as reserved title but standard takes precedence
            "RRQ" => Ok(MessageType::Rrq),
            "RRP" => Ok(MessageType::Rrp),
            "RRV" => Ok(MessageType::Rrv),
            "RTI" => Ok(MessageType::Rti),
            "SAM" => Ok(MessageType::Sam),
            "SBY" => Ok(MessageType::Sby),
            "SCO" => Ok(MessageType::Sco),
            "SDM" => Ok(MessageType::Sdm),
            "SIP" => Ok(MessageType::Sip),
            "SKC" => Ok(MessageType::Skc),
            "SLC" => Ok(MessageType::Slc),
            "SMM" => Ok(MessageType::Smm),
            "SPA" => Ok(MessageType::Spa),
            "SRJ" => Ok(MessageType::Srj),
            "SRM" => Ok(MessageType::Srm),
            "SWM" => Ok(MessageType::Swm),
            "TIM" => Ok(MessageType::Tim),
            "TIP" => Ok(MessageType::Tip),
            "UUP" => Ok(MessageType::Uup),
            "WAR" => Ok(MessageType::War),
            "XAP" => Ok(MessageType::Xap),
            "XCM" => Ok(MessageType::Xcm),
            "XIN" => Ok(MessageType::Xin),
            "XRQ" => Ok(MessageType::Xrq),
            "LOF" => Ok(MessageType::Logoff),
            // Reserved message titles
            "ACTDEP" => Ok(MessageType::Actdep),
            "ANM" => Ok(MessageType::Anm),
            "ANSWERCT" => Ok(MessageType::Answerct),
            "ANSWM" => Ok(MessageType::Answm),
            "ANSXFPLCT" => Ok(MessageType::Ansxfplct),
            "ATT" => Ok(MessageType::Att),
            "BEGINPROC" => Ok(MessageType::Beginproc),
            "BEGPROC" => Ok(MessageType::Begproc),
            "BEGPROCCT" => Ok(MessageType::Begprocct),
            "CDA" => Ok(MessageType::Cda),
            "CDAFTX" => Ok(MessageType::Cdaftx),
            "CHGDEP" => Ok(MessageType::Chgdep),
            "CHGMSG" => Ok(MessageType::Chgmsg),
            "CLD" => Ok(MessageType::Cld),
            "CLDFTX" => Ok(MessageType::Cldftx),
            "CNLARR" => Ok(MessageType::Cnlarr),
            "CNLCOND" => Ok(MessageType::Cnlcond),
            "CNLDEP" => Ok(MessageType::Cnldep),
            "CNLMSG" => Ok(MessageType::Cnlmsg),
            "CNLREG" => Ok(MessageType::Cnlreg),
            "CONFEND" => Ok(MessageType::Confend),
            "CONFIDM" => Ok(MessageType::Confidm),
            "CONFIDMCT" => Ok(MessageType::Confidmct),
            "CTARR" => Ok(MessageType::Ctarr),
            "DEC" => Ok(MessageType::Dec),
            "DOUBM" => Ok(MessageType::Doubm),
            "DRT" => Ok(MessageType::Drt),
            "EATARR" => Ok(MessageType::Eatarr),
            "ENDPROC" => Ok(MessageType::Endproc),
            "ENDPROCCT" => Ok(MessageType::Endprocct),
            "EVENT" => Ok(MessageType::Event),
            "EVLARR" => Ok(MessageType::Evlarr),
            "EVLDEP" => Ok(MessageType::Evldep),
            "EXCOND" => Ok(MessageType::Excond),
            "FICM" => Ok(MessageType::Ficm),
            "FLXVIVO" => Ok(MessageType::Flxvivo),
            "FPCLOSE" => Ok(MessageType::Fpclose),
            "FPCLOSECT" => Ok(MessageType::Fpclosect),
            "FPCLOSED" => Ok(MessageType::Fpclosed),
            "FPCRD" => Ok(MessageType::Fpcrd),
            "FPCRDCT" => Ok(MessageType::Fpcrdct),
            "FPCRDD" => Ok(MessageType::Fpcrdd),
            "FPCRE" => Ok(MessageType::Fpcre),
            "FPCRECT" => Ok(MessageType::Fpcrect),
            "FPINI" => Ok(MessageType::Fpini),
            "FPINICT" => Ok(MessageType::Fpinict),
            "FPINID" => Ok(MessageType::Fpinid),
            "FPLMSG" => Ok(MessageType::Fplmsg),
            "FPNTF" => Ok(MessageType::Fpntf),
            "FPNTFD" => Ok(MessageType::Fpntfd),
            "FPRDU" => Ok(MessageType::Fprdu),
            "FPRDUCT" => Ok(MessageType::Fprduct),
            "FSM" => Ok(MessageType::Fsm),
            "FSMFTX" => Ok(MessageType::Fsmftx),
            "FSR" => Ok(MessageType::Fsr),
            "IFPDQ" => Ok(MessageType::Ifpdq),
            "IFPDQR" => Ok(MessageType::Ifpdqr),
            "IFPDSQ" => Ok(MessageType::Ifpdsq),
            "IFPDSQR" => Ok(MessageType::Ifpdsqr),
            "INFOM" => Ok(MessageType::Infom),
            "IRQS" => Ok(MessageType::Irqs),
            "ISPL" => Ok(MessageType::Ispl),
            "LGR" => Ok(MessageType::Lgr),
            "LISTFP" => Ok(MessageType::Listfp),
            "LISTFPCT" => Ok(MessageType::Listfpct),
            "LOGON" => Ok(MessageType::LogonReserved),
            "MAJVIVO" => Ok(MessageType::Majvivo),
            "MCOM" => Ok(MessageType::Mcom),
            "MODCOND" => Ok(MessageType::Modcond),
            "MODREG" => Ok(MessageType::Modreg),
            "MRA" => Ok(MessageType::Mra),
            "MRCNL" => Ok(MessageType::Mrcnl),
            "MRMOD" => Ok(MessageType::Mrmod),
            "MRR" => Ok(MessageType::Mrr),
            "MVTVIVO" => Ok(MessageType::Mvtvivo),
            "NEWREG" => Ok(MessageType::Newreg),
            "NTA" => Ok(MessageType::Nta),
            "NTACNL" => Ok(MessageType::Ntacnl),
            "NTAMOD" => Ok(MessageType::Ntamod),
            "OCLM" => Ok(MessageType::Oclm),
            "OCLMD" => Ok(MessageType::Oclmd),
            "OLRA" => Ok(MessageType::Olra),
            "OLRCNL" => Ok(MessageType::Olrcnl),
            "OLRMOD" => Ok(MessageType::Olrmod),
            "PAMAER" => Ok(MessageType::Pamaer),
            "PAMARB" => Ok(MessageType::Pamarb),
            "PAMARRANN" => Ok(MessageType::Pamarrann),
            "PAMARRCRE" => Ok(MessageType::Pamarrcre),
            "PAMARRPST" => Ok(MessageType::Pamarrpst),
            "PAMDAPARB" => Ok(MessageType::Pamdaparb),
            "PAMDAPCRE" => Ok(MessageType::Pamdapcre),
            "PAMDEPANN" => Ok(MessageType::Pamdepann),
            "PAMDEPCRE" => Ok(MessageType::Pamdepcre),
            "PAMDEPPST" => Ok(MessageType::Pamdeppst),
            "PAMDRB" => Ok(MessageType::Pamdrb),
            "QTAARR" => Ok(MessageType::Qtaarr),
            "QTADEP" => Ok(MessageType::Qtadep),
            "RCD" => Ok(MessageType::Rcd),
            "RCDFTX" => Ok(MessageType::Rcdftx),
            "REVARR" => Ok(MessageType::Revarr),
            "RFPDQ" => Ok(MessageType::Rfpdq),
            "RFPDQR" => Ok(MessageType::Rfpdqr),
            "RFPDSQ" => Ok(MessageType::Rfpdsq),
            "RFPDSQR" => Ok(MessageType::Rfpdsqr),
            "RIEM" => Ok(MessageType::Riem),
            "RMG" => Ok(MessageType::Rmg),
            "RRA" => Ok(MessageType::Rra),
            "RREC" => Ok(MessageType::Rrec),
            // Note: RRN is handled as standard title above, not as reserved
            "RSUS" => Ok(MessageType::Rsus),
            "RWYCHGCT" => Ok(MessageType::Rwychgct),
            "RWYMSG" => Ok(MessageType::Rwymsg),
            "TRACT" => Ok(MessageType::Tract),
            "TRACTCT" => Ok(MessageType::Tractct),
            "TRCNL" => Ok(MessageType::Trcnl),
            "TRCNLCT" => Ok(MessageType::Trcnlct),
            "TRCOR" => Ok(MessageType::Trcor),
            "TRDECOR" => Ok(MessageType::Trdecor),
            "TRFIC" => Ok(MessageType::Trfic),
            "TRFICCT" => Ok(MessageType::Trficct),
            "TRFLRQT" => Ok(MessageType::Trflrqt),
            "TRMOD" => Ok(MessageType::Trmod),
            "TRMODCT" => Ok(MessageType::Trmodct),
            "TRMODH" => Ok(MessageType::Trmodh),
            "TRMODHD" => Ok(MessageType::Trmodhd),
            "TRMVT" => Ok(MessageType::Trmvt),
            "TRMVTCT" => Ok(MessageType::Trmvtct),
            "TRPOINT" => Ok(MessageType::Trpoint),
            "TRRET" => Ok(MessageType::Trret),
            "TRRETCT" => Ok(MessageType::Trretct),
            "TRRIP" => Ok(MessageType::Trrip),
            "TRRIPCT" => Ok(MessageType::Trripct),
            "TRRQT" => Ok(MessageType::Trrqt),
            "TRRQTCT" => Ok(MessageType::Trrqtct),
            "TRSHRQT" => Ok(MessageType::Trshrqt),
            "TRSTAR" => Ok(MessageType::Trstar),
            "TRSTARCT" => Ok(MessageType::Trstarct),
            "TRTRP" => Ok(MessageType::Trtrp),
            "TTIME" => Ok(MessageType::Ttime),
            "UNKFP" => Ok(MessageType::Unkfp),
            "UNKFPCT" => Ok(MessageType::Unkfpct),
            _ => Ok(MessageType::Generic),
        }
    }
    
    /// Retourne le préfixe du type
    pub fn prefix(&self) -> &str {
        match self {
            MessageType::FlightPlan => "FPL",
            MessageType::Change => "CHG",
            MessageType::Delay => "DLA",
            MessageType::Cancel => "CNL",
            MessageType::Departure => "DEP",
            MessageType::Arrival => "ARR",
            MessageType::Coordination => "COF",
            MessageType::Request => "REQ",
            MessageType::Estimate => "EST",
            MessageType::Position => "POS",
            MessageType::Logon => "LOG",
            // Standard message titles
            MessageType::Abi => "ABI",
            MessageType::Ack => "ACK",
            MessageType::Acp => "ACP",
            MessageType::Act => "ACT",
            MessageType::Ama => "AMA",
            MessageType::Apr => "APR",
            MessageType::Aup => "AUP",
            MessageType::Bfd => "BFD",
            MessageType::Cam => "CAM",
            MessageType::Ccm => "CCM",
            MessageType::Cdn => "CDN",
            MessageType::Cfd => "CFD",
            MessageType::Cod => "COD",
            MessageType::Cor => "COR",
            MessageType::Cram => "CRAM",
            MessageType::Cre => "CRE",
            MessageType::Crp => "CRP",
            MessageType::Crq => "CRQ",
            MessageType::Des => "DES",
            MessageType::Dpi => "DPI",
            MessageType::Efd => "EFD",
            MessageType::Err => "ERR",
            MessageType::Fcm => "FCM",
            MessageType::Fls => "FLS",
            MessageType::Fsa => "FSA",
            MessageType::Fum => "FUM",
            MessageType::Hop => "HOP",
            MessageType::Iach => "IACH",
            MessageType::Iafp => "IAFP",
            MessageType::Iapl => "IAPL",
            MessageType::Iarr => "IARR",
            MessageType::Ichg => "ICHG",
            MessageType::Icnl => "ICNL",
            MessageType::Idep => "IDEP",
            MessageType::Idla => "IDLA",
            MessageType::Ifpl => "IFPL",
            MessageType::Inf => "INF",
            MessageType::Irpl => "IRPL",
            MessageType::Irqp => "IRQP",
            MessageType::Lam => "LAM",
            MessageType::Lrm => "LRM",
            MessageType::Mac => "MAC",
            MessageType::Man => "MAN",
            MessageType::Mas => "MAS",
            MessageType::Nan => "NAN",
            MessageType::Ocm => "OCM",
            MessageType::Pac => "PAC",
            MessageType::Pnt => "PNT",
            MessageType::Rap => "RAP",
            MessageType::Rchg => "RCHG",
            MessageType::Rcl => "RCL",
            MessageType::Rcnl => "RCNL",
            MessageType::Rea => "REA",
            MessageType::Rej => "REJ",
            MessageType::Rev => "REV",
            MessageType::Rfi => "RFI",
            MessageType::Rjc => "RJC",
            MessageType::Rjt => "RJT",
            MessageType::Rls => "RLS",
            MessageType::Rof => "ROF",
            MessageType::Rrn => "RRN",  // Standard title
            // Note: RRN reserved title (RrnReserved) maps to same prefix but standard takes precedence
            MessageType::Rrq => "RRQ",
            MessageType::Rrp => "RRP",
            MessageType::Rrv => "RRV",
            MessageType::Rti => "RTI",
            MessageType::Sam => "SAM",
            MessageType::Sby => "SBY",
            MessageType::Sco => "SCO",
            MessageType::Sdm => "SDM",
            MessageType::Sip => "SIP",
            MessageType::Skc => "SKC",
            MessageType::Slc => "SLC",
            MessageType::Smm => "SMM",
            MessageType::Spa => "SPA",
            MessageType::Srj => "SRJ",
            MessageType::Srm => "SRM",
            MessageType::Swm => "SWM",
            MessageType::Tim => "TIM",
            MessageType::Tip => "TIP",
            MessageType::Uup => "UUP",
            MessageType::War => "WAR",
            MessageType::Xap => "XAP",
            MessageType::Xcm => "XCM",
            MessageType::Xin => "XIN",
            MessageType::Xrq => "XRQ",
            MessageType::Logoff => "LOF",
            // Reserved message titles
            MessageType::Actdep => "ACTDEP",
            MessageType::Anm => "ANM",
            MessageType::Answerct => "ANSWERCT",
            MessageType::Answm => "ANSWM",
            MessageType::Ansxfplct => "ANSXFPLCT",
            MessageType::Att => "ATT",
            MessageType::Beginproc => "BEGINPROC",
            MessageType::Begproc => "BEGPROC",
            MessageType::Begprocct => "BEGPROCCT",
            MessageType::Cda => "CDA",
            MessageType::Cdaftx => "CDAFTX",
            MessageType::Chgdep => "CHGDEP",
            MessageType::Chgmsg => "CHGMSG",
            MessageType::Cld => "CLD",
            MessageType::Cldftx => "CLDFTX",
            MessageType::Cnlarr => "CNLARR",
            MessageType::Cnlcond => "CNLCOND",
            MessageType::Cnldep => "CNLDEP",
            MessageType::Cnlmsg => "CNLMSG",
            MessageType::Cnlreg => "CNLREG",
            MessageType::Confend => "CONFEND",
            MessageType::Confidm => "CONFIDM",
            MessageType::Confidmct => "CONFIDMCT",
            MessageType::Ctarr => "CTARR",
            MessageType::Dec => "DEC",
            MessageType::Doubm => "DOUBM",
            MessageType::Drt => "DRT",
            MessageType::Eatarr => "EATARR",
            MessageType::Endproc => "ENDPROC",
            MessageType::Endprocct => "ENDPROCCT",
            MessageType::Event => "EVENT",
            MessageType::Evlarr => "EVLARR",
            MessageType::Evldep => "EVLDEP",
            MessageType::Excond => "EXCOND",
            MessageType::Ficm => "FICM",
            MessageType::Flxvivo => "FLXVIVO",
            MessageType::Fpclose => "FPCLOSE",
            MessageType::Fpclosect => "FPCLOSECT",
            MessageType::Fpclosed => "FPCLOSED",
            MessageType::Fpcrd => "FPCRD",
            MessageType::Fpcrdct => "FPCRDCT",
            MessageType::Fpcrdd => "FPCRDD",
            MessageType::Fpcre => "FPCRE",
            MessageType::Fpcrect => "FPCRECT",
            MessageType::Fpini => "FPINI",
            MessageType::Fpinict => "FPINICT",
            MessageType::Fpinid => "FPINID",
            MessageType::Fplmsg => "FPLMSG",
            MessageType::Fpntf => "FPNTF",
            MessageType::Fpntfd => "FPNTFD",
            MessageType::Fprdu => "FPRDU",
            MessageType::Fprduct => "FPRDUCT",
            MessageType::Fsm => "FSM",
            MessageType::Fsmftx => "FSMFTX",
            MessageType::Fsr => "FSR",
            MessageType::Ifpdq => "IFPDQ",
            MessageType::Ifpdqr => "IFPDQR",
            MessageType::Ifpdsq => "IFPDSQ",
            MessageType::Ifpdsqr => "IFPDSQR",
            MessageType::Infom => "INFOM",
            MessageType::Irqs => "IRQS",
            MessageType::Ispl => "ISPL",
            MessageType::Lgr => "LGR",
            MessageType::Listfp => "LISTFP",
            MessageType::Listfpct => "LISTFPCT",
            MessageType::LogonReserved => "LOGON",
            MessageType::Majvivo => "MAJVIVO",
            MessageType::Mcom => "MCOM",
            MessageType::Modcond => "MODCOND",
            MessageType::Modreg => "MODREG",
            MessageType::Mra => "MRA",
            MessageType::Mrcnl => "MRCNL",
            MessageType::Mrmod => "MRMOD",
            MessageType::Mrr => "MRR",
            MessageType::Mvtvivo => "MVTVIVO",
            MessageType::Newreg => "NEWREG",
            MessageType::Nta => "NTA",
            MessageType::Ntacnl => "NTACNL",
            MessageType::Ntamod => "NTAMOD",
            MessageType::Oclm => "OCLM",
            MessageType::Oclmd => "OCLMD",
            MessageType::Olra => "OLRA",
            MessageType::Olrcnl => "OLRCNL",
            MessageType::Olrmod => "OLRMOD",
            MessageType::Pamaer => "PAMAER",
            MessageType::Pamarb => "PAMARB",
            MessageType::Pamarrann => "PAMARRANN",
            MessageType::Pamarrcre => "PAMARRCRE",
            MessageType::Pamarrpst => "PAMARRPST",
            MessageType::Pamdaparb => "PAMDAPARB",
            MessageType::Pamdapcre => "PAMDAPCRE",
            MessageType::Pamdepann => "PAMDEPANN",
            MessageType::Pamdepcre => "PAMDEPCRE",
            MessageType::Pamdeppst => "PAMDEPPST",
            MessageType::Pamdrb => "PAMDRB",
            MessageType::Qtaarr => "QTAARR",
            MessageType::Qtadep => "QTADEP",
            MessageType::Rcd => "RCD",
            MessageType::Rcdftx => "RCDFTX",
            MessageType::Revarr => "REVARR",
            MessageType::Rfpdq => "RFPDQ",
            MessageType::Rfpdqr => "RFPDQR",
            MessageType::Rfpdsq => "RFPDSQ",
            MessageType::Rfpdsqr => "RFPDSQR",
            MessageType::Riem => "RIEM",
            MessageType::Rmg => "RMG",
            MessageType::Rra => "RRA",
            MessageType::Rrec => "RREC",
            // Note: RRN is handled as standard title above, not as reserved
            MessageType::RrnReserved => "RRN",  // Reserved title, but standard RRN takes precedence
            MessageType::Rsus => "RSUS",
            MessageType::Rwychgct => "RWYCHGCT",
            MessageType::Rwymsg => "RWYMSG",
            MessageType::Tract => "TRACT",
            MessageType::Tractct => "TRACTCT",
            MessageType::Trcnl => "TRCNL",
            MessageType::Trcnlct => "TRCNLCT",
            MessageType::Trcor => "TRCOR",
            MessageType::Trdecor => "TRDECOR",
            MessageType::Trfic => "TRFIC",
            MessageType::Trficct => "TRFICCT",
            MessageType::Trflrqt => "TRFLRQT",
            MessageType::Trmod => "TRMOD",
            MessageType::Trmodct => "TRMODCT",
            MessageType::Trmodh => "TRMODH",
            MessageType::Trmodhd => "TRMODHD",
            MessageType::Trmvt => "TRMVT",
            MessageType::Trmvtct => "TRMVTCT",
            MessageType::Trpoint => "TRPOINT",
            MessageType::Trret => "TRRET",
            MessageType::Trretct => "TRRETCT",
            MessageType::Trrip => "TRRIP",
            MessageType::Trripct => "TRRIPCT",
            MessageType::Trrqt => "TRRQT",
            MessageType::Trrqtct => "TRRQTCT",
            MessageType::Trshrqt => "TRSHRQT",
            MessageType::Trstar => "TRSTAR",
            MessageType::Trstarct => "TRSTARCT",
            MessageType::Trtrp => "TRTRP",
            MessageType::Ttime => "TTIME",
            MessageType::Unkfp => "UNKFP",
            MessageType::Unkfpct => "UNKFPCT",
            MessageType::Generic => "GEN",
        }
    }
}
