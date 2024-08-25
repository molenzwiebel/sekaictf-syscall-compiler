#![allow(dead_code, non_snake_case)]
use crate::insn::{Insn, ProgramBuilder, IRegOrImm, IReg};

impl ProgramBuilder {
    pub fn ZwAcceptConnectPort(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, PortContext: impl Into<IRegOrImm<u64>>, ConnectionRequest: impl Into<IRegOrImm<u64>>, AcceptConnection: impl Into<IRegOrImm<u64>>, ServerView: impl Into<IRegOrImm<u64>>, ClientView: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAcceptConnectPort", vec![PortHandle.into(), PortContext.into(), ConnectionRequest.into(), AcceptConnection.into(), ServerView.into(), ClientView.into()]));
        out
    }

    pub fn ZwAccessCheck(&mut self, SecurityDescriptor: impl Into<IRegOrImm<u64>>, ClientToken: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, GenericMapping: impl Into<IRegOrImm<u64>>, PrivilegeSet: impl Into<IRegOrImm<u64>>, PrivilegeSetLength: impl Into<IRegOrImm<u64>>, GrantedAccess: impl Into<IRegOrImm<u64>>, AccessStatus: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAccessCheck", vec![SecurityDescriptor.into(), ClientToken.into(), DesiredAccess.into(), GenericMapping.into(), PrivilegeSet.into(), PrivilegeSetLength.into(), GrantedAccess.into(), AccessStatus.into()]));
        out
    }

    pub fn ZwAccessCheckAndAuditAlarm(&mut self, SubsystemName: impl Into<IRegOrImm<u64>>, HandleId: impl Into<IRegOrImm<u64>>, ObjectTypeName: impl Into<IRegOrImm<u64>>, ObjectName: impl Into<IRegOrImm<u64>>, SecurityDescriptor: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, GenericMapping: impl Into<IRegOrImm<u64>>, ObjectCreation: impl Into<IRegOrImm<u64>>, GrantedAccess: impl Into<IRegOrImm<u64>>, AccessStatus: impl Into<IRegOrImm<u64>>, GenerateOnClose: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAccessCheckAndAuditAlarm", vec![SubsystemName.into(), HandleId.into(), ObjectTypeName.into(), ObjectName.into(), SecurityDescriptor.into(), DesiredAccess.into(), GenericMapping.into(), ObjectCreation.into(), GrantedAccess.into(), AccessStatus.into(), GenerateOnClose.into()]));
        out
    }

    pub fn ZwAccessCheckByType(&mut self, SecurityDescriptor: impl Into<IRegOrImm<u64>>, PrincipalSelfSid: impl Into<IRegOrImm<u64>>, ClientToken: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectTypeList: impl Into<IRegOrImm<u64>>, ObjectTypeListLength: impl Into<IRegOrImm<u64>>, GenericMapping: impl Into<IRegOrImm<u64>>, PrivilegeSet: impl Into<IRegOrImm<u64>>, PrivilegeSetLength: impl Into<IRegOrImm<u64>>, GrantedAccess: impl Into<IRegOrImm<u64>>, AccessStatus: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAccessCheckByType", vec![SecurityDescriptor.into(), PrincipalSelfSid.into(), ClientToken.into(), DesiredAccess.into(), ObjectTypeList.into(), ObjectTypeListLength.into(), GenericMapping.into(), PrivilegeSet.into(), PrivilegeSetLength.into(), GrantedAccess.into(), AccessStatus.into()]));
        out
    }

    pub fn ZwAccessCheckByTypeAndAuditAlarm(&mut self, SubsystemName: impl Into<IRegOrImm<u64>>, HandleId: impl Into<IRegOrImm<u64>>, ObjectTypeName: impl Into<IRegOrImm<u64>>, ObjectName: impl Into<IRegOrImm<u64>>, SecurityDescriptor: impl Into<IRegOrImm<u64>>, PrincipalSelfSid: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, AuditType: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, ObjectTypeList: impl Into<IRegOrImm<u64>>, ObjectTypeListLength: impl Into<IRegOrImm<u64>>, GenericMapping: impl Into<IRegOrImm<u64>>, ObjectCreation: impl Into<IRegOrImm<u64>>, GrantedAccess: impl Into<IRegOrImm<u64>>, AccessStatus: impl Into<IRegOrImm<u64>>, GenerateOnClose: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAccessCheckByTypeAndAuditAlarm", vec![SubsystemName.into(), HandleId.into(), ObjectTypeName.into(), ObjectName.into(), SecurityDescriptor.into(), PrincipalSelfSid.into(), DesiredAccess.into(), AuditType.into(), Flags.into(), ObjectTypeList.into(), ObjectTypeListLength.into(), GenericMapping.into(), ObjectCreation.into(), GrantedAccess.into(), AccessStatus.into(), GenerateOnClose.into()]));
        out
    }

    pub fn ZwAccessCheckByTypeResultList(&mut self, SecurityDescriptor: impl Into<IRegOrImm<u64>>, PrincipalSelfSid: impl Into<IRegOrImm<u64>>, ClientToken: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectTypeList: impl Into<IRegOrImm<u64>>, ObjectTypeListLength: impl Into<IRegOrImm<u64>>, GenericMapping: impl Into<IRegOrImm<u64>>, PrivilegeSet: impl Into<IRegOrImm<u64>>, PrivilegeSetLength: impl Into<IRegOrImm<u64>>, GrantedAccess: impl Into<IRegOrImm<u64>>, AccessStatus: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAccessCheckByTypeResultList", vec![SecurityDescriptor.into(), PrincipalSelfSid.into(), ClientToken.into(), DesiredAccess.into(), ObjectTypeList.into(), ObjectTypeListLength.into(), GenericMapping.into(), PrivilegeSet.into(), PrivilegeSetLength.into(), GrantedAccess.into(), AccessStatus.into()]));
        out
    }

    pub fn ZwAccessCheckByTypeResultListAndAuditAlarm(&mut self, SubsystemName: impl Into<IRegOrImm<u64>>, HandleId: impl Into<IRegOrImm<u64>>, ObjectTypeName: impl Into<IRegOrImm<u64>>, ObjectName: impl Into<IRegOrImm<u64>>, SecurityDescriptor: impl Into<IRegOrImm<u64>>, PrincipalSelfSid: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, AuditType: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, ObjectTypeList: impl Into<IRegOrImm<u64>>, ObjectTypeListLength: impl Into<IRegOrImm<u64>>, GenericMapping: impl Into<IRegOrImm<u64>>, ObjectCreation: impl Into<IRegOrImm<u64>>, GrantedAccess: impl Into<IRegOrImm<u64>>, AccessStatus: impl Into<IRegOrImm<u64>>, GenerateOnClose: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAccessCheckByTypeResultListAndAuditAlarm", vec![SubsystemName.into(), HandleId.into(), ObjectTypeName.into(), ObjectName.into(), SecurityDescriptor.into(), PrincipalSelfSid.into(), DesiredAccess.into(), AuditType.into(), Flags.into(), ObjectTypeList.into(), ObjectTypeListLength.into(), GenericMapping.into(), ObjectCreation.into(), GrantedAccess.into(), AccessStatus.into(), GenerateOnClose.into()]));
        out
    }

    pub fn ZwAccessCheckByTypeResultListAndAuditAlarmByHandle(&mut self, SubsystemName: impl Into<IRegOrImm<u64>>, HandleId: impl Into<IRegOrImm<u64>>, ClientToken: impl Into<IRegOrImm<u64>>, ObjectTypeName: impl Into<IRegOrImm<u64>>, ObjectName: impl Into<IRegOrImm<u64>>, SecurityDescriptor: impl Into<IRegOrImm<u64>>, PrincipalSelfSid: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, AuditType: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, ObjectTypeList: impl Into<IRegOrImm<u64>>, ObjectTypeListLength: impl Into<IRegOrImm<u64>>, GenericMapping: impl Into<IRegOrImm<u64>>, ObjectCreation: impl Into<IRegOrImm<u64>>, GrantedAccess: impl Into<IRegOrImm<u64>>, AccessStatus: impl Into<IRegOrImm<u64>>, GenerateOnClose: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAccessCheckByTypeResultListAndAuditAlarmByHandle", vec![SubsystemName.into(), HandleId.into(), ClientToken.into(), ObjectTypeName.into(), ObjectName.into(), SecurityDescriptor.into(), PrincipalSelfSid.into(), DesiredAccess.into(), AuditType.into(), Flags.into(), ObjectTypeList.into(), ObjectTypeListLength.into(), GenericMapping.into(), ObjectCreation.into(), GrantedAccess.into(), AccessStatus.into(), GenerateOnClose.into()]));
        out
    }

    pub fn ZwAcquireCMFViewOwnership(&mut self, TimeStamp: impl Into<IRegOrImm<u64>>, tokenTaken: impl Into<IRegOrImm<u64>>, replaceExisting: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAcquireCMFViewOwnership", vec![TimeStamp.into(), tokenTaken.into(), replaceExisting.into()]));
        out
    }

    pub fn ZwAddAtom(&mut self, AtomName: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, Atom: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAddAtom", vec![AtomName.into(), Length.into(), Atom.into()]));
        out
    }

    pub fn ZwAddAtomEx(&mut self, AtomName: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, Atom: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAddAtomEx", vec![AtomName.into(), Length.into(), Atom.into(), Flags.into()]));
        out
    }

    pub fn ZwAddBootEntry(&mut self, BootEntry: impl Into<IRegOrImm<u64>>, Id: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAddBootEntry", vec![BootEntry.into(), Id.into()]));
        out
    }

    pub fn ZwAddDriverEntry(&mut self, DriverEntry: impl Into<IRegOrImm<u64>>, Id: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAddDriverEntry", vec![DriverEntry.into(), Id.into()]));
        out
    }

    pub fn ZwAdjustGroupsToken(&mut self, TokenHandle: impl Into<IRegOrImm<u64>>, ResetToDefault: impl Into<IRegOrImm<u64>>, NewState: impl Into<IRegOrImm<u64>>, BufferLength: impl Into<IRegOrImm<u64>>, PreviousState: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAdjustGroupsToken", vec![TokenHandle.into(), ResetToDefault.into(), NewState.into(), BufferLength.into(), PreviousState.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwAdjustPrivilegesToken(&mut self, TokenHandle: impl Into<IRegOrImm<u64>>, DisableAllPrivileges: impl Into<IRegOrImm<u64>>, NewState: impl Into<IRegOrImm<u64>>, BufferLength: impl Into<IRegOrImm<u64>>, PreviousState: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAdjustPrivilegesToken", vec![TokenHandle.into(), DisableAllPrivileges.into(), NewState.into(), BufferLength.into(), PreviousState.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwAdjustTokenClaimsAndDeviceGroups(&mut self, TokenHandle: impl Into<IRegOrImm<u64>>, UserResetToDefault: impl Into<IRegOrImm<u64>>, DeviceResetToDefault: impl Into<IRegOrImm<u64>>, DeviceGroupsResetToDefault: impl Into<IRegOrImm<u64>>, NewUserState: impl Into<IRegOrImm<u64>>, NewDeviceState: impl Into<IRegOrImm<u64>>, NewDeviceGroupsState: impl Into<IRegOrImm<u64>>, UserBufferLength: impl Into<IRegOrImm<u64>>, PreviousUserState: impl Into<IRegOrImm<u64>>, DeviceBufferLength: impl Into<IRegOrImm<u64>>, PreviousDeviceState: impl Into<IRegOrImm<u64>>, DeviceGroupsBufferLength: impl Into<IRegOrImm<u64>>, PreviousDeviceGroups: impl Into<IRegOrImm<u64>>, UserReturnLength: impl Into<IRegOrImm<u64>>, DeviceReturnLength: impl Into<IRegOrImm<u64>>, DeviceGroupsReturnBufferLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAdjustTokenClaimsAndDeviceGroups", vec![TokenHandle.into(), UserResetToDefault.into(), DeviceResetToDefault.into(), DeviceGroupsResetToDefault.into(), NewUserState.into(), NewDeviceState.into(), NewDeviceGroupsState.into(), UserBufferLength.into(), PreviousUserState.into(), DeviceBufferLength.into(), PreviousDeviceState.into(), DeviceGroupsBufferLength.into(), PreviousDeviceGroups.into(), UserReturnLength.into(), DeviceReturnLength.into(), DeviceGroupsReturnBufferLength.into()]));
        out
    }

    pub fn ZwAlertResumeThread(&mut self, ThreadHandle: impl Into<IRegOrImm<u64>>, PreviousSuspendCount: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlertResumeThread", vec![ThreadHandle.into(), PreviousSuspendCount.into()]));
        out
    }

    pub fn ZwAlertThread(&mut self, ThreadHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlertThread", vec![ThreadHandle.into()]));
        out
    }

    pub fn ZwAlertThreadByThreadId(&mut self, ThreadId: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlertThreadByThreadId", vec![ThreadId.into()]));
        out
    }

    pub fn ZwAllocateLocallyUniqueId(&mut self, Luid: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAllocateLocallyUniqueId", vec![Luid.into()]));
        out
    }

    pub fn ZwAllocateReserveObject(&mut self, MemoryReserveHandle: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, Type: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAllocateReserveObject", vec![MemoryReserveHandle.into(), ObjectAttributes.into(), Type.into()]));
        out
    }

    pub fn ZwAllocateUserPhysicalPages(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, NumberOfPages: impl Into<IRegOrImm<u64>>, UserPfnArray: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAllocateUserPhysicalPages", vec![ProcessHandle.into(), NumberOfPages.into(), UserPfnArray.into()]));
        out
    }

    pub fn ZwAllocateUserPhysicalPagesEx(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, NumberOfPages: impl Into<IRegOrImm<u64>>, UserPfnArray: impl Into<IRegOrImm<u64>>, ExtendedParameters: impl Into<IRegOrImm<u64>>, ExtendedParameterCount: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAllocateUserPhysicalPagesEx", vec![ProcessHandle.into(), NumberOfPages.into(), UserPfnArray.into(), ExtendedParameters.into(), ExtendedParameterCount.into()]));
        out
    }

    pub fn ZwAllocateUuids(&mut self, Time: impl Into<IRegOrImm<u64>>, Range: impl Into<IRegOrImm<u64>>, Sequence: impl Into<IRegOrImm<u64>>, Seed: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAllocateUuids", vec![Time.into(), Range.into(), Sequence.into(), Seed.into()]));
        out
    }

    pub fn ZwAllocateVirtualMemory(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, BaseAddress: impl Into<IRegOrImm<u64>>, ZeroBits: impl Into<IRegOrImm<u64>>, RegionSize: impl Into<IRegOrImm<u64>>, AllocationType: impl Into<IRegOrImm<u64>>, Protect: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAllocateVirtualMemory", vec![ProcessHandle.into(), BaseAddress.into(), ZeroBits.into(), RegionSize.into(), AllocationType.into(), Protect.into()]));
        out
    }

    pub fn ZwAllocateVirtualMemoryEx(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, BaseAddress: impl Into<IRegOrImm<u64>>, RegionSize: impl Into<IRegOrImm<u64>>, AllocationType: impl Into<IRegOrImm<u64>>, PageProtection: impl Into<IRegOrImm<u64>>, ExtendedParameters: impl Into<IRegOrImm<u64>>, ExtendedParameterCount: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAllocateVirtualMemoryEx", vec![ProcessHandle.into(), BaseAddress.into(), RegionSize.into(), AllocationType.into(), PageProtection.into(), ExtendedParameters.into(), ExtendedParameterCount.into()]));
        out
    }

    pub fn ZwAlpcAcceptConnectPort(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, ConnectionPortHandle: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, PortAttributes: impl Into<IRegOrImm<u64>>, PortContext: impl Into<IRegOrImm<u64>>, ConnectionRequest: impl Into<IRegOrImm<u64>>, ConnectionMessageAttributes: impl Into<IRegOrImm<u64>>, AcceptConnection: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlpcAcceptConnectPort", vec![PortHandle.into(), ConnectionPortHandle.into(), Flags.into(), ObjectAttributes.into(), PortAttributes.into(), PortContext.into(), ConnectionRequest.into(), ConnectionMessageAttributes.into(), AcceptConnection.into()]));
        out
    }

    pub fn ZwAlpcCancelMessage(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, MessageContext: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlpcCancelMessage", vec![PortHandle.into(), Flags.into(), MessageContext.into()]));
        out
    }

    pub fn ZwAlpcConnectPort(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, PortName: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, PortAttributes: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, RequiredServerSid: impl Into<IRegOrImm<u64>>, ConnectionMessage: impl Into<IRegOrImm<u64>>, BufferLength: impl Into<IRegOrImm<u64>>, OutMessageAttributes: impl Into<IRegOrImm<u64>>, InMessageAttributes: impl Into<IRegOrImm<u64>>, Timeout: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlpcConnectPort", vec![PortHandle.into(), PortName.into(), ObjectAttributes.into(), PortAttributes.into(), Flags.into(), RequiredServerSid.into(), ConnectionMessage.into(), BufferLength.into(), OutMessageAttributes.into(), InMessageAttributes.into(), Timeout.into()]));
        out
    }

    pub fn ZwAlpcConnectPortEx(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, ConnectionPortObjectAttributes: impl Into<IRegOrImm<u64>>, ClientPortObjectAttributes: impl Into<IRegOrImm<u64>>, PortAttributes: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, ServerSecurityRequirements: impl Into<IRegOrImm<u64>>, ConnectionMessage: impl Into<IRegOrImm<u64>>, BufferLength: impl Into<IRegOrImm<u64>>, OutMessageAttributes: impl Into<IRegOrImm<u64>>, InMessageAttributes: impl Into<IRegOrImm<u64>>, Timeout: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlpcConnectPortEx", vec![PortHandle.into(), ConnectionPortObjectAttributes.into(), ClientPortObjectAttributes.into(), PortAttributes.into(), Flags.into(), ServerSecurityRequirements.into(), ConnectionMessage.into(), BufferLength.into(), OutMessageAttributes.into(), InMessageAttributes.into(), Timeout.into()]));
        out
    }

    pub fn ZwAlpcCreatePort(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, PortAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlpcCreatePort", vec![PortHandle.into(), ObjectAttributes.into(), PortAttributes.into()]));
        out
    }

    pub fn ZwAlpcCreatePortSection(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, SectionHandle: impl Into<IRegOrImm<u64>>, SectionSize: impl Into<IRegOrImm<u64>>, AlpcSectionHandle: impl Into<IRegOrImm<u64>>, ActualSectionSize: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlpcCreatePortSection", vec![PortHandle.into(), Flags.into(), SectionHandle.into(), SectionSize.into(), AlpcSectionHandle.into(), ActualSectionSize.into()]));
        out
    }

    pub fn ZwAlpcCreateResourceReserve(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, MessageSize: impl Into<IRegOrImm<u64>>, ResourceId: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlpcCreateResourceReserve", vec![PortHandle.into(), Flags.into(), MessageSize.into(), ResourceId.into()]));
        out
    }

    pub fn ZwAlpcCreateSectionView(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, ViewAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlpcCreateSectionView", vec![PortHandle.into(), Flags.into(), ViewAttributes.into()]));
        out
    }

    pub fn ZwAlpcCreateSecurityContext(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, SecurityAttribute: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlpcCreateSecurityContext", vec![PortHandle.into(), Flags.into(), SecurityAttribute.into()]));
        out
    }

    pub fn ZwAlpcDeletePortSection(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, SectionHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlpcDeletePortSection", vec![PortHandle.into(), Flags.into(), SectionHandle.into()]));
        out
    }

    pub fn ZwAlpcDeleteResourceReserve(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, ResourceId: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlpcDeleteResourceReserve", vec![PortHandle.into(), Flags.into(), ResourceId.into()]));
        out
    }

    pub fn ZwAlpcDeleteSectionView(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, ViewBase: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlpcDeleteSectionView", vec![PortHandle.into(), Flags.into(), ViewBase.into()]));
        out
    }

    pub fn ZwAlpcDeleteSecurityContext(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, ContextHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlpcDeleteSecurityContext", vec![PortHandle.into(), Flags.into(), ContextHandle.into()]));
        out
    }

    pub fn ZwAlpcDisconnectPort(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlpcDisconnectPort", vec![PortHandle.into(), Flags.into()]));
        out
    }

    pub fn ZwAlpcImpersonateClientContainerOfPort(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, Message: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlpcImpersonateClientContainerOfPort", vec![PortHandle.into(), Message.into(), Flags.into()]));
        out
    }

    pub fn ZwAlpcImpersonateClientOfPort(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, Message: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlpcImpersonateClientOfPort", vec![PortHandle.into(), Message.into(), Flags.into()]));
        out
    }

    pub fn ZwAlpcOpenSenderProcess(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, PortHandle: impl Into<IRegOrImm<u64>>, PortMessage: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlpcOpenSenderProcess", vec![ProcessHandle.into(), PortHandle.into(), PortMessage.into(), Flags.into(), DesiredAccess.into(), ObjectAttributes.into()]));
        out
    }

    pub fn ZwAlpcOpenSenderThread(&mut self, ThreadHandle: impl Into<IRegOrImm<u64>>, PortHandle: impl Into<IRegOrImm<u64>>, PortMessage: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlpcOpenSenderThread", vec![ThreadHandle.into(), PortHandle.into(), PortMessage.into(), Flags.into(), DesiredAccess.into(), ObjectAttributes.into()]));
        out
    }

    pub fn ZwAlpcQueryInformation(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, PortInformationClass: impl Into<IRegOrImm<u64>>, PortInformation: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlpcQueryInformation", vec![PortHandle.into(), PortInformationClass.into(), PortInformation.into(), Length.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwAlpcQueryInformationMessage(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, PortMessage: impl Into<IRegOrImm<u64>>, MessageInformationClass: impl Into<IRegOrImm<u64>>, MessageInformation: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlpcQueryInformationMessage", vec![PortHandle.into(), PortMessage.into(), MessageInformationClass.into(), MessageInformation.into(), Length.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwAlpcRevokeSecurityContext(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, ContextHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlpcRevokeSecurityContext", vec![PortHandle.into(), Flags.into(), ContextHandle.into()]));
        out
    }

    pub fn ZwAlpcSendWaitReceivePort(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, SendMessage: impl Into<IRegOrImm<u64>>, SendMessageAttributes: impl Into<IRegOrImm<u64>>, ReceiveMessage: impl Into<IRegOrImm<u64>>, BufferLength: impl Into<IRegOrImm<u64>>, ReceiveMessageAttributes: impl Into<IRegOrImm<u64>>, Timeout: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlpcSendWaitReceivePort", vec![PortHandle.into(), Flags.into(), SendMessage.into(), SendMessageAttributes.into(), ReceiveMessage.into(), BufferLength.into(), ReceiveMessageAttributes.into(), Timeout.into()]));
        out
    }

    pub fn ZwAlpcSetInformation(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, PortInformationClass: impl Into<IRegOrImm<u64>>, PortInformation: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAlpcSetInformation", vec![PortHandle.into(), PortInformationClass.into(), PortInformation.into(), Length.into()]));
        out
    }

    pub fn ZwAreMappedFilesTheSame(&mut self, File1MappedAsAnImage: impl Into<IRegOrImm<u64>>, File2MappedAsFile: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAreMappedFilesTheSame", vec![File1MappedAsAnImage.into(), File2MappedAsFile.into()]));
        out
    }

    pub fn ZwAssignProcessToJobObject(&mut self, JobHandle: impl Into<IRegOrImm<u64>>, ProcessHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAssignProcessToJobObject", vec![JobHandle.into(), ProcessHandle.into()]));
        out
    }

    pub fn ZwAssociateWaitCompletionPacket(&mut self, WaitCompletionPacketHandle: impl Into<IRegOrImm<u64>>, IoCompletionHandle: impl Into<IRegOrImm<u64>>, TargetObjectHandle: impl Into<IRegOrImm<u64>>, KeyContext: impl Into<IRegOrImm<u64>>, ApcContext: impl Into<IRegOrImm<u64>>, IoStatus: impl Into<IRegOrImm<u64>>, IoStatusInformation: impl Into<IRegOrImm<u64>>, AlreadySignaled: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwAssociateWaitCompletionPacket", vec![WaitCompletionPacketHandle.into(), IoCompletionHandle.into(), TargetObjectHandle.into(), KeyContext.into(), ApcContext.into(), IoStatus.into(), IoStatusInformation.into(), AlreadySignaled.into()]));
        out
    }

    pub fn ZwCallbackReturn(&mut self, OutputBuffer: impl Into<IRegOrImm<u64>>, OutputLength: impl Into<IRegOrImm<u64>>, Status: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCallbackReturn", vec![OutputBuffer.into(), OutputLength.into(), Status.into()]));
        out
    }

    pub fn ZwCallEnclave(&mut self, Routine: impl Into<IRegOrImm<u64>>, Reserved: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, RoutineParamReturn: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCallEnclave", vec![Routine.into(), Reserved.into(), Flags.into(), RoutineParamReturn.into()]));
        out
    }

    pub fn ZwCancelIoFile(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCancelIoFile", vec![FileHandle.into(), IoStatusBlock.into()]));
        out
    }

    pub fn ZwCancelIoFileEx(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, IoRequestToCancel: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCancelIoFileEx", vec![FileHandle.into(), IoRequestToCancel.into(), IoStatusBlock.into()]));
        out
    }

    pub fn ZwCancelSynchronousIoFile(&mut self, ThreadHandle: impl Into<IRegOrImm<u64>>, IoRequestToCancel: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCancelSynchronousIoFile", vec![ThreadHandle.into(), IoRequestToCancel.into(), IoStatusBlock.into()]));
        out
    }

    pub fn ZwCancelTimer(&mut self, TimerHandle: impl Into<IRegOrImm<u64>>, CurrentState: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCancelTimer", vec![TimerHandle.into(), CurrentState.into()]));
        out
    }

    pub fn ZwCancelTimer2(&mut self, TimerHandle: impl Into<IRegOrImm<u64>>, Parameters: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCancelTimer2", vec![TimerHandle.into(), Parameters.into()]));
        out
    }

    pub fn ZwCancelWaitCompletionPacket(&mut self, WaitCompletionPacketHandle: impl Into<IRegOrImm<u64>>, RemoveSignaledPacket: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCancelWaitCompletionPacket", vec![WaitCompletionPacketHandle.into(), RemoveSignaledPacket.into()]));
        out
    }

    pub fn ZwChangeProcessState(&mut self, ProcessStateChangeHandle: impl Into<IRegOrImm<u64>>, ProcessHandle: impl Into<IRegOrImm<u64>>, StateChangeType: impl Into<IRegOrImm<u64>>, ExtendedInformation: impl Into<IRegOrImm<u64>>, ExtendedInformationLength: impl Into<IRegOrImm<u64>>, Reserved: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwChangeProcessState", vec![ProcessStateChangeHandle.into(), ProcessHandle.into(), StateChangeType.into(), ExtendedInformation.into(), ExtendedInformationLength.into(), Reserved.into()]));
        out
    }

    pub fn ZwChangeThreadState(&mut self, ThreadStateChangeHandle: impl Into<IRegOrImm<u64>>, ThreadHandle: impl Into<IRegOrImm<u64>>, StateChangeType: impl Into<IRegOrImm<u64>>, ExtendedInformation: impl Into<IRegOrImm<u64>>, ExtendedInformationLength: impl Into<IRegOrImm<u64>>, Reserved: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwChangeThreadState", vec![ThreadStateChangeHandle.into(), ThreadHandle.into(), StateChangeType.into(), ExtendedInformation.into(), ExtendedInformationLength.into(), Reserved.into()]));
        out
    }

    pub fn ZwClearEvent(&mut self, EventHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwClearEvent", vec![EventHandle.into()]));
        out
    }

    pub fn ZwClose(&mut self, Handle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwClose", vec![Handle.into()]));
        out
    }

    pub fn ZwCloseObjectAuditAlarm(&mut self, SubsystemName: impl Into<IRegOrImm<u64>>, HandleId: impl Into<IRegOrImm<u64>>, GenerateOnClose: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCloseObjectAuditAlarm", vec![SubsystemName.into(), HandleId.into(), GenerateOnClose.into()]));
        out
    }

    pub fn ZwCommitComplete(&mut self, EnlistmentHandle: impl Into<IRegOrImm<u64>>, TmVirtualClock: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCommitComplete", vec![EnlistmentHandle.into(), TmVirtualClock.into()]));
        out
    }

    pub fn ZwCommitEnlistment(&mut self, EnlistmentHandle: impl Into<IRegOrImm<u64>>, TmVirtualClock: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCommitEnlistment", vec![EnlistmentHandle.into(), TmVirtualClock.into()]));
        out
    }

    pub fn ZwCommitTransaction(&mut self, TransactionHandle: impl Into<IRegOrImm<u64>>, Wait: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCommitTransaction", vec![TransactionHandle.into(), Wait.into()]));
        out
    }

    pub fn ZwCompactKeys(&mut self, Count: impl Into<IRegOrImm<u64>>, KeyArray: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCompactKeys", vec![Count.into(), KeyArray.into()]));
        out
    }

    pub fn ZwCompareObjects(&mut self, FirstObjectHandle: impl Into<IRegOrImm<u64>>, SecondObjectHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCompareObjects", vec![FirstObjectHandle.into(), SecondObjectHandle.into()]));
        out
    }

    pub fn ZwCompareSigningLevels(&mut self, FirstSigningLevel: impl Into<IRegOrImm<u64>>, SecondSigningLevel: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCompareSigningLevels", vec![FirstSigningLevel.into(), SecondSigningLevel.into()]));
        out
    }

    pub fn ZwCompareTokens(&mut self, FirstTokenHandle: impl Into<IRegOrImm<u64>>, SecondTokenHandle: impl Into<IRegOrImm<u64>>, Equal: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCompareTokens", vec![FirstTokenHandle.into(), SecondTokenHandle.into(), Equal.into()]));
        out
    }

    pub fn ZwCompleteConnectPort(&mut self, PortHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCompleteConnectPort", vec![PortHandle.into()]));
        out
    }

    pub fn ZwCompressKey(&mut self, KeyHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCompressKey", vec![KeyHandle.into()]));
        out
    }

    pub fn ZwConnectPort(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, PortName: impl Into<IRegOrImm<u64>>, SecurityQos: impl Into<IRegOrImm<u64>>, ClientView: impl Into<IRegOrImm<u64>>, ServerView: impl Into<IRegOrImm<u64>>, MaxMessageLength: impl Into<IRegOrImm<u64>>, ConnectionInformation: impl Into<IRegOrImm<u64>>, ConnectionInformationLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwConnectPort", vec![PortHandle.into(), PortName.into(), SecurityQos.into(), ClientView.into(), ServerView.into(), MaxMessageLength.into(), ConnectionInformation.into(), ConnectionInformationLength.into()]));
        out
    }

    pub fn ZwContinue(&mut self, ContextRecord: impl Into<IRegOrImm<u64>>, TestAlert: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwContinue", vec![ContextRecord.into(), TestAlert.into()]));
        out
    }

    pub fn ZwContinueEx(&mut self, ContextRecord: impl Into<IRegOrImm<u64>>, ContinueArgument: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwContinueEx", vec![ContextRecord.into(), ContinueArgument.into()]));
        out
    }

    pub fn ZwConvertBetweenAuxiliaryCounterAndPerformanceCounter(&mut self, ConvertAuxiliaryToPerformanceCounter: impl Into<IRegOrImm<u64>>, PerformanceOrAuxiliaryCounterValue: impl Into<IRegOrImm<u64>>, ConvertedValue: impl Into<IRegOrImm<u64>>, ConversionError: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwConvertBetweenAuxiliaryCounterAndPerformanceCounter", vec![ConvertAuxiliaryToPerformanceCounter.into(), PerformanceOrAuxiliaryCounterValue.into(), ConvertedValue.into(), ConversionError.into()]));
        out
    }

    pub fn ZwCopyFileChunk(&mut self, SourceHandle: impl Into<IRegOrImm<u64>>, DestinationHandle: impl Into<IRegOrImm<u64>>, EventHandle: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, SourceOffset: impl Into<IRegOrImm<u64>>, DestOffset: impl Into<IRegOrImm<u64>>, SourceKey: impl Into<IRegOrImm<u64>>, DestKey: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCopyFileChunk", vec![SourceHandle.into(), DestinationHandle.into(), EventHandle.into(), IoStatusBlock.into(), Length.into(), SourceOffset.into(), DestOffset.into(), SourceKey.into(), DestKey.into(), Flags.into()]));
        out
    }

    pub fn ZwCreateDebugObject(&mut self, DebugObjectHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateDebugObject", vec![DebugObjectHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), Flags.into()]));
        out
    }

    pub fn ZwCreateDirectoryObject(&mut self, DirectoryHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateDirectoryObject", vec![DirectoryHandle.into(), DesiredAccess.into(), ObjectAttributes.into()]));
        out
    }

    pub fn ZwCreateDirectoryObjectEx(&mut self, DirectoryHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, ShadowDirectoryHandle: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateDirectoryObjectEx", vec![DirectoryHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), ShadowDirectoryHandle.into(), Flags.into()]));
        out
    }

    pub fn ZwCreateEnclave(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, BaseAddress: impl Into<IRegOrImm<u64>>, ZeroBits: impl Into<IRegOrImm<u64>>, Size: impl Into<IRegOrImm<u64>>, InitialCommitment: impl Into<IRegOrImm<u64>>, EnclaveType: impl Into<IRegOrImm<u64>>, EnclaveInformation: impl Into<IRegOrImm<u64>>, EnclaveInformationLength: impl Into<IRegOrImm<u64>>, EnclaveError: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateEnclave", vec![ProcessHandle.into(), BaseAddress.into(), ZeroBits.into(), Size.into(), InitialCommitment.into(), EnclaveType.into(), EnclaveInformation.into(), EnclaveInformationLength.into(), EnclaveError.into()]));
        out
    }

    pub fn ZwCreateEnlistment(&mut self, EnlistmentHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ResourceManagerHandle: impl Into<IRegOrImm<u64>>, TransactionHandle: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, CreateOptions: impl Into<IRegOrImm<u64>>, NotificationMask: impl Into<IRegOrImm<u64>>, EnlistmentKey: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateEnlistment", vec![EnlistmentHandle.into(), DesiredAccess.into(), ResourceManagerHandle.into(), TransactionHandle.into(), ObjectAttributes.into(), CreateOptions.into(), NotificationMask.into(), EnlistmentKey.into()]));
        out
    }

    pub fn ZwCreateEvent(&mut self, EventHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, EventType: impl Into<IRegOrImm<u64>>, InitialState: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateEvent", vec![EventHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), EventType.into(), InitialState.into()]));
        out
    }

    pub fn ZwCreateEventPair(&mut self, EventPairHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateEventPair", vec![EventPairHandle.into(), DesiredAccess.into(), ObjectAttributes.into()]));
        out
    }

    pub fn ZwCreateFile(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, AllocationSize: impl Into<IRegOrImm<u64>>, FileAttributes: impl Into<IRegOrImm<u64>>, ShareAccess: impl Into<IRegOrImm<u64>>, CreateDisposition: impl Into<IRegOrImm<u64>>, CreateOptions: impl Into<IRegOrImm<u64>>, EaBuffer: impl Into<IRegOrImm<u64>>, EaLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateFile", vec![FileHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), IoStatusBlock.into(), AllocationSize.into(), FileAttributes.into(), ShareAccess.into(), CreateDisposition.into(), CreateOptions.into(), EaBuffer.into(), EaLength.into()]));
        out
    }

    pub fn ZwCreateIoCompletion(&mut self, IoCompletionHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, Count: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateIoCompletion", vec![IoCompletionHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), Count.into()]));
        out
    }

    pub fn ZwCreateIoRing(&mut self, IoRingHandle: impl Into<IRegOrImm<u64>>, CreateParametersLength: impl Into<IRegOrImm<u64>>, CreateParameters: impl Into<IRegOrImm<u64>>, OutputParametersLength: impl Into<IRegOrImm<u64>>, OutputParameters: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateIoRing", vec![IoRingHandle.into(), CreateParametersLength.into(), CreateParameters.into(), OutputParametersLength.into(), OutputParameters.into()]));
        out
    }

    pub fn ZwCreateIRTimer(&mut self, TimerHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateIRTimer", vec![TimerHandle.into(), DesiredAccess.into()]));
        out
    }

    pub fn ZwCreateJobObject(&mut self, JobHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateJobObject", vec![JobHandle.into(), DesiredAccess.into(), ObjectAttributes.into()]));
        out
    }

    pub fn ZwCreateJobSet(&mut self, NumJob: impl Into<IRegOrImm<u64>>, UserJobSet: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateJobSet", vec![NumJob.into(), UserJobSet.into(), Flags.into()]));
        out
    }

    pub fn ZwCreateKey(&mut self, KeyHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, TitleIndex: impl Into<IRegOrImm<u64>>, Class: impl Into<IRegOrImm<u64>>, CreateOptions: impl Into<IRegOrImm<u64>>, Disposition: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateKey", vec![KeyHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), TitleIndex.into(), Class.into(), CreateOptions.into(), Disposition.into()]));
        out
    }

    pub fn ZwCreateKeyedEvent(&mut self, KeyedEventHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateKeyedEvent", vec![KeyedEventHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), Flags.into()]));
        out
    }

    pub fn ZwCreateKeyTransacted(&mut self, KeyHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, TitleIndex: impl Into<IRegOrImm<u64>>, Class: impl Into<IRegOrImm<u64>>, CreateOptions: impl Into<IRegOrImm<u64>>, TransactionHandle: impl Into<IRegOrImm<u64>>, Disposition: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateKeyTransacted", vec![KeyHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), TitleIndex.into(), Class.into(), CreateOptions.into(), TransactionHandle.into(), Disposition.into()]));
        out
    }

    pub fn ZwCreateLowBoxToken(&mut self, TokenHandle: impl Into<IRegOrImm<u64>>, ExistingTokenHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, PackageSid: impl Into<IRegOrImm<u64>>, CapabilityCount: impl Into<IRegOrImm<u64>>, Capabilities: impl Into<IRegOrImm<u64>>, HandleCount: impl Into<IRegOrImm<u64>>, Handles: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateLowBoxToken", vec![TokenHandle.into(), ExistingTokenHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), PackageSid.into(), CapabilityCount.into(), Capabilities.into(), HandleCount.into(), Handles.into()]));
        out
    }

    pub fn ZwCreateMailslotFile(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, CreateOptions: impl Into<IRegOrImm<u64>>, MailslotQuota: impl Into<IRegOrImm<u64>>, MaximumMessageSize: impl Into<IRegOrImm<u64>>, ReadTimeout: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateMailslotFile", vec![FileHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), IoStatusBlock.into(), CreateOptions.into(), MailslotQuota.into(), MaximumMessageSize.into(), ReadTimeout.into()]));
        out
    }

    pub fn ZwCreateMutant(&mut self, MutantHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, InitialOwner: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateMutant", vec![MutantHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), InitialOwner.into()]));
        out
    }

    pub fn ZwCreateNamedPipeFile(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, ShareAccess: impl Into<IRegOrImm<u64>>, CreateDisposition: impl Into<IRegOrImm<u64>>, CreateOptions: impl Into<IRegOrImm<u64>>, NamedPipeType: impl Into<IRegOrImm<u64>>, ReadMode: impl Into<IRegOrImm<u64>>, CompletionMode: impl Into<IRegOrImm<u64>>, MaximumInstances: impl Into<IRegOrImm<u64>>, InboundQuota: impl Into<IRegOrImm<u64>>, OutboundQuota: impl Into<IRegOrImm<u64>>, DefaultTimeout: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateNamedPipeFile", vec![FileHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), IoStatusBlock.into(), ShareAccess.into(), CreateDisposition.into(), CreateOptions.into(), NamedPipeType.into(), ReadMode.into(), CompletionMode.into(), MaximumInstances.into(), InboundQuota.into(), OutboundQuota.into(), DefaultTimeout.into()]));
        out
    }

    pub fn ZwCreatePagingFile(&mut self, PageFileName: impl Into<IRegOrImm<u64>>, MinimumSize: impl Into<IRegOrImm<u64>>, MaximumSize: impl Into<IRegOrImm<u64>>, Priority: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreatePagingFile", vec![PageFileName.into(), MinimumSize.into(), MaximumSize.into(), Priority.into()]));
        out
    }

    pub fn ZwCreatePartition(&mut self, ParentPartitionHandle: impl Into<IRegOrImm<u64>>, PartitionHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, PreferredNode: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreatePartition", vec![ParentPartitionHandle.into(), PartitionHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), PreferredNode.into()]));
        out
    }

    pub fn ZwCreatePort(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, MaxConnectionInfoLength: impl Into<IRegOrImm<u64>>, MaxMessageLength: impl Into<IRegOrImm<u64>>, MaxPoolUsage: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreatePort", vec![PortHandle.into(), ObjectAttributes.into(), MaxConnectionInfoLength.into(), MaxMessageLength.into(), MaxPoolUsage.into()]));
        out
    }

    pub fn ZwCreatePrivateNamespace(&mut self, NamespaceHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, BoundaryDescriptor: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreatePrivateNamespace", vec![NamespaceHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), BoundaryDescriptor.into()]));
        out
    }

    pub fn ZwCreateProcess(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, ParentProcess: impl Into<IRegOrImm<u64>>, InheritObjectTable: impl Into<IRegOrImm<u64>>, SectionHandle: impl Into<IRegOrImm<u64>>, DebugPort: impl Into<IRegOrImm<u64>>, TokenHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateProcess", vec![ProcessHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), ParentProcess.into(), InheritObjectTable.into(), SectionHandle.into(), DebugPort.into(), TokenHandle.into()]));
        out
    }

    pub fn ZwCreateProcessEx(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, ParentProcess: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, SectionHandle: impl Into<IRegOrImm<u64>>, DebugPort: impl Into<IRegOrImm<u64>>, TokenHandle: impl Into<IRegOrImm<u64>>, Reserved: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateProcessEx", vec![ProcessHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), ParentProcess.into(), Flags.into(), SectionHandle.into(), DebugPort.into(), TokenHandle.into(), Reserved.into()]));
        out
    }

    pub fn ZwCreateProcessStateChange(&mut self, ProcessStateChangeHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, ProcessHandle: impl Into<IRegOrImm<u64>>, Reserved: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateProcessStateChange", vec![ProcessStateChangeHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), ProcessHandle.into(), Reserved.into()]));
        out
    }

    pub fn ZwCreateProfile(&mut self, ProfileHandle: impl Into<IRegOrImm<u64>>, Process: impl Into<IRegOrImm<u64>>, ProfileBase: impl Into<IRegOrImm<u64>>, ProfileSize: impl Into<IRegOrImm<u64>>, BucketSize: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>, BufferSize: impl Into<IRegOrImm<u64>>, ProfileSource: impl Into<IRegOrImm<u64>>, Affinity: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateProfile", vec![ProfileHandle.into(), Process.into(), ProfileBase.into(), ProfileSize.into(), BucketSize.into(), Buffer.into(), BufferSize.into(), ProfileSource.into(), Affinity.into()]));
        out
    }

    pub fn ZwCreateProfileEx(&mut self, ProfileHandle: impl Into<IRegOrImm<u64>>, Process: impl Into<IRegOrImm<u64>>, ProfileBase: impl Into<IRegOrImm<u64>>, ProfileSize: impl Into<IRegOrImm<u64>>, BucketSize: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>, BufferSize: impl Into<IRegOrImm<u64>>, ProfileSource: impl Into<IRegOrImm<u64>>, GroupCount: impl Into<IRegOrImm<u64>>, GroupAffinity: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateProfileEx", vec![ProfileHandle.into(), Process.into(), ProfileBase.into(), ProfileSize.into(), BucketSize.into(), Buffer.into(), BufferSize.into(), ProfileSource.into(), GroupCount.into(), GroupAffinity.into()]));
        out
    }

    pub fn ZwCreateResourceManager(&mut self, ResourceManagerHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, TmHandle: impl Into<IRegOrImm<u64>>, RmGuid: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, CreateOptions: impl Into<IRegOrImm<u64>>, Description: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateResourceManager", vec![ResourceManagerHandle.into(), DesiredAccess.into(), TmHandle.into(), RmGuid.into(), ObjectAttributes.into(), CreateOptions.into(), Description.into()]));
        out
    }

    pub fn ZwCreateSection(&mut self, SectionHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, MaximumSize: impl Into<IRegOrImm<u64>>, SectionPageProtection: impl Into<IRegOrImm<u64>>, AllocationAttributes: impl Into<IRegOrImm<u64>>, FileHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateSection", vec![SectionHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), MaximumSize.into(), SectionPageProtection.into(), AllocationAttributes.into(), FileHandle.into()]));
        out
    }

    pub fn ZwCreateSectionEx(&mut self, SectionHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, MaximumSize: impl Into<IRegOrImm<u64>>, SectionPageProtection: impl Into<IRegOrImm<u64>>, AllocationAttributes: impl Into<IRegOrImm<u64>>, FileHandle: impl Into<IRegOrImm<u64>>, ExtendedParameters: impl Into<IRegOrImm<u64>>, ExtendedParameterCount: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateSectionEx", vec![SectionHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), MaximumSize.into(), SectionPageProtection.into(), AllocationAttributes.into(), FileHandle.into(), ExtendedParameters.into(), ExtendedParameterCount.into()]));
        out
    }

    pub fn ZwCreateSemaphore(&mut self, SemaphoreHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, InitialCount: impl Into<IRegOrImm<u64>>, MaximumCount: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateSemaphore", vec![SemaphoreHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), InitialCount.into(), MaximumCount.into()]));
        out
    }

    pub fn ZwCreateSymbolicLinkObject(&mut self, LinkHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, LinkTarget: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateSymbolicLinkObject", vec![LinkHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), LinkTarget.into()]));
        out
    }

    pub fn ZwCreateThread(&mut self, ThreadHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, ProcessHandle: impl Into<IRegOrImm<u64>>, ClientId: impl Into<IRegOrImm<u64>>, ThreadContext: impl Into<IRegOrImm<u64>>, InitialTeb: impl Into<IRegOrImm<u64>>, CreateSuspended: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateThread", vec![ThreadHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), ProcessHandle.into(), ClientId.into(), ThreadContext.into(), InitialTeb.into(), CreateSuspended.into()]));
        out
    }

    pub fn ZwCreateThreadEx(&mut self, ThreadHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, ProcessHandle: impl Into<IRegOrImm<u64>>, StartRoutine: impl Into<IRegOrImm<u64>>, Argument: impl Into<IRegOrImm<u64>>, CreateFlags: impl Into<IRegOrImm<u64>>, ZeroBits: impl Into<IRegOrImm<u64>>, StackSize: impl Into<IRegOrImm<u64>>, MaximumStackSize: impl Into<IRegOrImm<u64>>, AttributeList: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateThreadEx", vec![ThreadHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), ProcessHandle.into(), StartRoutine.into(), Argument.into(), CreateFlags.into(), ZeroBits.into(), StackSize.into(), MaximumStackSize.into(), AttributeList.into()]));
        out
    }

    pub fn ZwCreateThreadStateChange(&mut self, ThreadStateChangeHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, ThreadHandle: impl Into<IRegOrImm<u64>>, Reserved: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateThreadStateChange", vec![ThreadStateChangeHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), ThreadHandle.into(), Reserved.into()]));
        out
    }

    pub fn ZwCreateTimer(&mut self, TimerHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, TimerType: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateTimer", vec![TimerHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), TimerType.into()]));
        out
    }

    pub fn ZwCreateTimer2(&mut self, TimerHandle: impl Into<IRegOrImm<u64>>, Reserved1: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, Attributes: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateTimer2", vec![TimerHandle.into(), Reserved1.into(), ObjectAttributes.into(), Attributes.into(), DesiredAccess.into()]));
        out
    }

    pub fn ZwCreateToken(&mut self, TokenHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, Type: impl Into<IRegOrImm<u64>>, AuthenticationId: impl Into<IRegOrImm<u64>>, ExpirationTime: impl Into<IRegOrImm<u64>>, User: impl Into<IRegOrImm<u64>>, Groups: impl Into<IRegOrImm<u64>>, Privileges: impl Into<IRegOrImm<u64>>, Owner: impl Into<IRegOrImm<u64>>, PrimaryGroup: impl Into<IRegOrImm<u64>>, DefaultDacl: impl Into<IRegOrImm<u64>>, Source: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateToken", vec![TokenHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), Type.into(), AuthenticationId.into(), ExpirationTime.into(), User.into(), Groups.into(), Privileges.into(), Owner.into(), PrimaryGroup.into(), DefaultDacl.into(), Source.into()]));
        out
    }

    pub fn ZwCreateTokenEx(&mut self, TokenHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, Type: impl Into<IRegOrImm<u64>>, AuthenticationId: impl Into<IRegOrImm<u64>>, ExpirationTime: impl Into<IRegOrImm<u64>>, User: impl Into<IRegOrImm<u64>>, Groups: impl Into<IRegOrImm<u64>>, Privileges: impl Into<IRegOrImm<u64>>, UserAttributes: impl Into<IRegOrImm<u64>>, DeviceAttributes: impl Into<IRegOrImm<u64>>, DeviceGroups: impl Into<IRegOrImm<u64>>, MandatoryPolicy: impl Into<IRegOrImm<u64>>, Owner: impl Into<IRegOrImm<u64>>, PrimaryGroup: impl Into<IRegOrImm<u64>>, DefaultDacl: impl Into<IRegOrImm<u64>>, Source: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateTokenEx", vec![TokenHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), Type.into(), AuthenticationId.into(), ExpirationTime.into(), User.into(), Groups.into(), Privileges.into(), UserAttributes.into(), DeviceAttributes.into(), DeviceGroups.into(), MandatoryPolicy.into(), Owner.into(), PrimaryGroup.into(), DefaultDacl.into(), Source.into()]));
        out
    }

    pub fn ZwCreateTransaction(&mut self, TransactionHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, Uow: impl Into<IRegOrImm<u64>>, TmHandle: impl Into<IRegOrImm<u64>>, CreateOptions: impl Into<IRegOrImm<u64>>, IsolationLevel: impl Into<IRegOrImm<u64>>, IsolationFlags: impl Into<IRegOrImm<u64>>, Timeout: impl Into<IRegOrImm<u64>>, Description: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateTransaction", vec![TransactionHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), Uow.into(), TmHandle.into(), CreateOptions.into(), IsolationLevel.into(), IsolationFlags.into(), Timeout.into(), Description.into()]));
        out
    }

    pub fn ZwCreateTransactionManager(&mut self, TmHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, LogFileName: impl Into<IRegOrImm<u64>>, CreateOptions: impl Into<IRegOrImm<u64>>, CommitStrength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateTransactionManager", vec![TmHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), LogFileName.into(), CreateOptions.into(), CommitStrength.into()]));
        out
    }

    pub fn ZwCreateUserProcess(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, ThreadHandle: impl Into<IRegOrImm<u64>>, ProcessDesiredAccess: impl Into<IRegOrImm<u64>>, ThreadDesiredAccess: impl Into<IRegOrImm<u64>>, ProcessObjectAttributes: impl Into<IRegOrImm<u64>>, ThreadObjectAttributes: impl Into<IRegOrImm<u64>>, ProcessFlags: impl Into<IRegOrImm<u64>>, ThreadFlags: impl Into<IRegOrImm<u64>>, ProcessParameters: impl Into<IRegOrImm<u64>>, CreateInfo: impl Into<IRegOrImm<u64>>, AttributeList: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateUserProcess", vec![ProcessHandle.into(), ThreadHandle.into(), ProcessDesiredAccess.into(), ThreadDesiredAccess.into(), ProcessObjectAttributes.into(), ThreadObjectAttributes.into(), ProcessFlags.into(), ThreadFlags.into(), ProcessParameters.into(), CreateInfo.into(), AttributeList.into()]));
        out
    }

    pub fn ZwCreateWaitablePort(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, MaxConnectionInfoLength: impl Into<IRegOrImm<u64>>, MaxMessageLength: impl Into<IRegOrImm<u64>>, MaxPoolUsage: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateWaitablePort", vec![PortHandle.into(), ObjectAttributes.into(), MaxConnectionInfoLength.into(), MaxMessageLength.into(), MaxPoolUsage.into()]));
        out
    }

    pub fn ZwCreateWaitCompletionPacket(&mut self, WaitCompletionPacketHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateWaitCompletionPacket", vec![WaitCompletionPacketHandle.into(), DesiredAccess.into(), ObjectAttributes.into()]));
        out
    }

    pub fn ZwCreateWnfStateName(&mut self, StateName: impl Into<IRegOrImm<u64>>, NameLifetime: impl Into<IRegOrImm<u64>>, DataScope: impl Into<IRegOrImm<u64>>, PersistData: impl Into<IRegOrImm<u64>>, TypeId: impl Into<IRegOrImm<u64>>, MaximumStateSize: impl Into<IRegOrImm<u64>>, SecurityDescriptor: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateWnfStateName", vec![StateName.into(), NameLifetime.into(), DataScope.into(), PersistData.into(), TypeId.into(), MaximumStateSize.into(), SecurityDescriptor.into()]));
        out
    }

    pub fn ZwCreateWorkerFactory(&mut self, WorkerFactoryHandleReturn: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, CompletionPortHandle: impl Into<IRegOrImm<u64>>, WorkerProcessHandle: impl Into<IRegOrImm<u64>>, StartRoutine: impl Into<IRegOrImm<u64>>, StartParameter: impl Into<IRegOrImm<u64>>, MaxThreadCount: impl Into<IRegOrImm<u64>>, StackReserve: impl Into<IRegOrImm<u64>>, StackCommit: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwCreateWorkerFactory", vec![WorkerFactoryHandleReturn.into(), DesiredAccess.into(), ObjectAttributes.into(), CompletionPortHandle.into(), WorkerProcessHandle.into(), StartRoutine.into(), StartParameter.into(), MaxThreadCount.into(), StackReserve.into(), StackCommit.into()]));
        out
    }

    pub fn ZwDebugActiveProcess(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, DebugObjectHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwDebugActiveProcess", vec![ProcessHandle.into(), DebugObjectHandle.into()]));
        out
    }

    pub fn ZwDebugContinue(&mut self, DebugObjectHandle: impl Into<IRegOrImm<u64>>, ClientId: impl Into<IRegOrImm<u64>>, ContinueStatus: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwDebugContinue", vec![DebugObjectHandle.into(), ClientId.into(), ContinueStatus.into()]));
        out
    }

    pub fn ZwDelayExecution(&mut self, Alertable: impl Into<IRegOrImm<u64>>, DelayInterval: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwDelayExecution", vec![Alertable.into(), DelayInterval.into()]));
        out
    }

    pub fn ZwDeleteAtom(&mut self, Atom: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwDeleteAtom", vec![Atom.into()]));
        out
    }

    pub fn ZwDeleteBootEntry(&mut self, Id: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwDeleteBootEntry", vec![Id.into()]));
        out
    }

    pub fn ZwDeleteDriverEntry(&mut self, Id: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwDeleteDriverEntry", vec![Id.into()]));
        out
    }

    pub fn ZwDeleteFile(&mut self, ObjectAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwDeleteFile", vec![ObjectAttributes.into()]));
        out
    }

    pub fn ZwDeleteKey(&mut self, KeyHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwDeleteKey", vec![KeyHandle.into()]));
        out
    }

    pub fn ZwDeleteObjectAuditAlarm(&mut self, SubsystemName: impl Into<IRegOrImm<u64>>, HandleId: impl Into<IRegOrImm<u64>>, GenerateOnClose: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwDeleteObjectAuditAlarm", vec![SubsystemName.into(), HandleId.into(), GenerateOnClose.into()]));
        out
    }

    pub fn ZwDeletePrivateNamespace(&mut self, NamespaceHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwDeletePrivateNamespace", vec![NamespaceHandle.into()]));
        out
    }

    pub fn ZwDeleteValueKey(&mut self, KeyHandle: impl Into<IRegOrImm<u64>>, ValueName: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwDeleteValueKey", vec![KeyHandle.into(), ValueName.into()]));
        out
    }

    pub fn ZwDeleteWnfStateData(&mut self, StateName: impl Into<IRegOrImm<u64>>, ExplicitScope: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwDeleteWnfStateData", vec![StateName.into(), ExplicitScope.into()]));
        out
    }

    pub fn ZwDeleteWnfStateName(&mut self, StateName: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwDeleteWnfStateName", vec![StateName.into()]));
        out
    }

    pub fn ZwDeviceIoControlFile(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, Event: impl Into<IRegOrImm<u64>>, ApcRoutine: impl Into<IRegOrImm<u64>>, ApcContext: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, IoControlCode: impl Into<IRegOrImm<u64>>, InputBuffer: impl Into<IRegOrImm<u64>>, InputBufferLength: impl Into<IRegOrImm<u64>>, OutputBuffer: impl Into<IRegOrImm<u64>>, OutputBufferLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwDeviceIoControlFile", vec![FileHandle.into(), Event.into(), ApcRoutine.into(), ApcContext.into(), IoStatusBlock.into(), IoControlCode.into(), InputBuffer.into(), InputBufferLength.into(), OutputBuffer.into(), OutputBufferLength.into()]));
        out
    }

    pub fn ZwDisableLastKnownGood(&mut self, ) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwDisableLastKnownGood", vec![]));
        out
    }

    pub fn ZwDisplayString(&mut self, String: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwDisplayString", vec![String.into()]));
        out
    }

    pub fn ZwDrawText(&mut self, Text: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwDrawText", vec![Text.into()]));
        out
    }

    pub fn ZwDuplicateObject(&mut self, SourceProcessHandle: impl Into<IRegOrImm<u64>>, SourceHandle: impl Into<IRegOrImm<u64>>, TargetProcessHandle: impl Into<IRegOrImm<u64>>, TargetHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, HandleAttributes: impl Into<IRegOrImm<u64>>, Options: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwDuplicateObject", vec![SourceProcessHandle.into(), SourceHandle.into(), TargetProcessHandle.into(), TargetHandle.into(), DesiredAccess.into(), HandleAttributes.into(), Options.into()]));
        out
    }

    pub fn ZwDuplicateToken(&mut self, ExistingTokenHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, EffectiveOnly: impl Into<IRegOrImm<u64>>, Type: impl Into<IRegOrImm<u64>>, NewTokenHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwDuplicateToken", vec![ExistingTokenHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), EffectiveOnly.into(), Type.into(), NewTokenHandle.into()]));
        out
    }

    pub fn ZwEnableLastKnownGood(&mut self, ) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwEnableLastKnownGood", vec![]));
        out
    }

    pub fn ZwEnumerateBootEntries(&mut self, Buffer: impl Into<IRegOrImm<u64>>, BufferLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwEnumerateBootEntries", vec![Buffer.into(), BufferLength.into()]));
        out
    }

    pub fn ZwEnumerateDriverEntries(&mut self, Buffer: impl Into<IRegOrImm<u64>>, BufferLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwEnumerateDriverEntries", vec![Buffer.into(), BufferLength.into()]));
        out
    }

    pub fn ZwEnumerateKey(&mut self, KeyHandle: impl Into<IRegOrImm<u64>>, Index: impl Into<IRegOrImm<u64>>, KeyInformationClass: impl Into<IRegOrImm<u64>>, KeyInformation: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, ResultLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwEnumerateKey", vec![KeyHandle.into(), Index.into(), KeyInformationClass.into(), KeyInformation.into(), Length.into(), ResultLength.into()]));
        out
    }

    pub fn ZwEnumerateSystemEnvironmentValuesEx(&mut self, InformationClass: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>, BufferLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwEnumerateSystemEnvironmentValuesEx", vec![InformationClass.into(), Buffer.into(), BufferLength.into()]));
        out
    }

    pub fn ZwEnumerateTransactionObject(&mut self, RootObjectHandle: impl Into<IRegOrImm<u64>>, QueryType: impl Into<IRegOrImm<u64>>, ObjectCursor: impl Into<IRegOrImm<u64>>, ObjectCursorLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwEnumerateTransactionObject", vec![RootObjectHandle.into(), QueryType.into(), ObjectCursor.into(), ObjectCursorLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwEnumerateValueKey(&mut self, KeyHandle: impl Into<IRegOrImm<u64>>, Index: impl Into<IRegOrImm<u64>>, KeyValueInformationClass: impl Into<IRegOrImm<u64>>, KeyValueInformation: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, ResultLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwEnumerateValueKey", vec![KeyHandle.into(), Index.into(), KeyValueInformationClass.into(), KeyValueInformation.into(), Length.into(), ResultLength.into()]));
        out
    }

    pub fn ZwExtendSection(&mut self, SectionHandle: impl Into<IRegOrImm<u64>>, NewSectionSize: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwExtendSection", vec![SectionHandle.into(), NewSectionSize.into()]));
        out
    }

    pub fn ZwFilterBootOption(&mut self, FilterOperation: impl Into<IRegOrImm<u64>>, ObjectType: impl Into<IRegOrImm<u64>>, ElementType: impl Into<IRegOrImm<u64>>, Data: impl Into<IRegOrImm<u64>>, DataSize: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwFilterBootOption", vec![FilterOperation.into(), ObjectType.into(), ElementType.into(), Data.into(), DataSize.into()]));
        out
    }

    pub fn ZwFilterToken(&mut self, ExistingTokenHandle: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, SidsToDisable: impl Into<IRegOrImm<u64>>, PrivilegesToDelete: impl Into<IRegOrImm<u64>>, RestrictedSids: impl Into<IRegOrImm<u64>>, NewTokenHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwFilterToken", vec![ExistingTokenHandle.into(), Flags.into(), SidsToDisable.into(), PrivilegesToDelete.into(), RestrictedSids.into(), NewTokenHandle.into()]));
        out
    }

    pub fn ZwFilterTokenEx(&mut self, ExistingTokenHandle: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, SidsToDisable: impl Into<IRegOrImm<u64>>, PrivilegesToDelete: impl Into<IRegOrImm<u64>>, RestrictedSids: impl Into<IRegOrImm<u64>>, DisableUserClaimsCount: impl Into<IRegOrImm<u64>>, UserClaimsToDisable: impl Into<IRegOrImm<u64>>, DisableDeviceClaimsCount: impl Into<IRegOrImm<u64>>, DeviceClaimsToDisable: impl Into<IRegOrImm<u64>>, DeviceGroupsToDisable: impl Into<IRegOrImm<u64>>, RestrictedUserAttributes: impl Into<IRegOrImm<u64>>, RestrictedDeviceAttributes: impl Into<IRegOrImm<u64>>, RestrictedDeviceGroups: impl Into<IRegOrImm<u64>>, NewTokenHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwFilterTokenEx", vec![ExistingTokenHandle.into(), Flags.into(), SidsToDisable.into(), PrivilegesToDelete.into(), RestrictedSids.into(), DisableUserClaimsCount.into(), UserClaimsToDisable.into(), DisableDeviceClaimsCount.into(), DeviceClaimsToDisable.into(), DeviceGroupsToDisable.into(), RestrictedUserAttributes.into(), RestrictedDeviceAttributes.into(), RestrictedDeviceGroups.into(), NewTokenHandle.into()]));
        out
    }

    pub fn ZwFindAtom(&mut self, AtomName: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, Atom: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwFindAtom", vec![AtomName.into(), Length.into(), Atom.into()]));
        out
    }

    pub fn ZwFlushBuffersFile(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwFlushBuffersFile", vec![FileHandle.into(), IoStatusBlock.into()]));
        out
    }

    pub fn ZwFlushBuffersFileEx(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, Parameters: impl Into<IRegOrImm<u64>>, ParametersSize: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwFlushBuffersFileEx", vec![FileHandle.into(), Flags.into(), Parameters.into(), ParametersSize.into(), IoStatusBlock.into()]));
        out
    }

    pub fn ZwFlushInstallUILanguage(&mut self, InstallUILanguage: impl Into<IRegOrImm<u64>>, SetComittedFlag: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwFlushInstallUILanguage", vec![InstallUILanguage.into(), SetComittedFlag.into()]));
        out
    }

    pub fn ZwFlushInstructionCache(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, BaseAddress: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwFlushInstructionCache", vec![ProcessHandle.into(), BaseAddress.into(), Length.into()]));
        out
    }

    pub fn ZwFlushKey(&mut self, KeyHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwFlushKey", vec![KeyHandle.into()]));
        out
    }

    pub fn ZwFlushProcessWriteBuffers(&mut self, ) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwFlushProcessWriteBuffers", vec![]));
        out
    }

    pub fn ZwFlushVirtualMemory(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, BaseAddress: impl Into<IRegOrImm<u64>>, RegionSize: impl Into<IRegOrImm<u64>>, IoStatus: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwFlushVirtualMemory", vec![ProcessHandle.into(), BaseAddress.into(), RegionSize.into(), IoStatus.into()]));
        out
    }

    pub fn ZwFlushWriteBuffer(&mut self, ) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwFlushWriteBuffer", vec![]));
        out
    }

    pub fn ZwFreeUserPhysicalPages(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, NumberOfPages: impl Into<IRegOrImm<u64>>, UserPfnArray: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwFreeUserPhysicalPages", vec![ProcessHandle.into(), NumberOfPages.into(), UserPfnArray.into()]));
        out
    }

    pub fn ZwFreeVirtualMemory(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, BaseAddress: impl Into<IRegOrImm<u64>>, RegionSize: impl Into<IRegOrImm<u64>>, FreeType: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwFreeVirtualMemory", vec![ProcessHandle.into(), BaseAddress.into(), RegionSize.into(), FreeType.into()]));
        out
    }

    pub fn ZwFreezeRegistry(&mut self, TimeOutInSeconds: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwFreezeRegistry", vec![TimeOutInSeconds.into()]));
        out
    }

    pub fn ZwFreezeTransactions(&mut self, FreezeTimeout: impl Into<IRegOrImm<u64>>, ThawTimeout: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwFreezeTransactions", vec![FreezeTimeout.into(), ThawTimeout.into()]));
        out
    }

    pub fn ZwFsControlFile(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, Event: impl Into<IRegOrImm<u64>>, ApcRoutine: impl Into<IRegOrImm<u64>>, ApcContext: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, FsControlCode: impl Into<IRegOrImm<u64>>, InputBuffer: impl Into<IRegOrImm<u64>>, InputBufferLength: impl Into<IRegOrImm<u64>>, OutputBuffer: impl Into<IRegOrImm<u64>>, OutputBufferLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwFsControlFile", vec![FileHandle.into(), Event.into(), ApcRoutine.into(), ApcContext.into(), IoStatusBlock.into(), FsControlCode.into(), InputBuffer.into(), InputBufferLength.into(), OutputBuffer.into(), OutputBufferLength.into()]));
        out
    }

    pub fn ZwGetCachedSigningLevel(&mut self, File: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, SigningLevel: impl Into<IRegOrImm<u64>>, Thumbprint: impl Into<IRegOrImm<u64>>, ThumbprintSize: impl Into<IRegOrImm<u64>>, ThumbprintAlgorithm: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwGetCachedSigningLevel", vec![File.into(), Flags.into(), SigningLevel.into(), Thumbprint.into(), ThumbprintSize.into(), ThumbprintAlgorithm.into()]));
        out
    }

    pub fn ZwGetCompleteWnfStateSubscription(&mut self, OldDescriptorStateName: impl Into<IRegOrImm<u64>>, OldSubscriptionId: impl Into<IRegOrImm<u64>>, OldDescriptorEventMask: impl Into<IRegOrImm<u64>>, OldDescriptorStatus: impl Into<IRegOrImm<u64>>, NewDeliveryDescriptor: impl Into<IRegOrImm<u64>>, DescriptorSize: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwGetCompleteWnfStateSubscription", vec![OldDescriptorStateName.into(), OldSubscriptionId.into(), OldDescriptorEventMask.into(), OldDescriptorStatus.into(), NewDeliveryDescriptor.into(), DescriptorSize.into()]));
        out
    }

    pub fn ZwGetContextThread(&mut self, ThreadHandle: impl Into<IRegOrImm<u64>>, ThreadContext: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwGetContextThread", vec![ThreadHandle.into(), ThreadContext.into()]));
        out
    }

    pub fn ZwGetCurrentProcessorNumber(&mut self, ) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwGetCurrentProcessorNumber", vec![]));
        out
    }

    pub fn ZwGetCurrentProcessorNumberEx(&mut self, ProcessorNumber: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwGetCurrentProcessorNumberEx", vec![ProcessorNumber.into()]));
        out
    }

    pub fn ZwGetDevicePowerState(&mut self, Device: impl Into<IRegOrImm<u64>>, State: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwGetDevicePowerState", vec![Device.into(), State.into()]));
        out
    }

    pub fn ZwGetMUIRegistryInfo(&mut self, Flags: impl Into<IRegOrImm<u64>>, DataSize: impl Into<IRegOrImm<u64>>, Data: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwGetMUIRegistryInfo", vec![Flags.into(), DataSize.into(), Data.into()]));
        out
    }

    pub fn ZwGetNextProcess(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, HandleAttributes: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, NewProcessHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwGetNextProcess", vec![ProcessHandle.into(), DesiredAccess.into(), HandleAttributes.into(), Flags.into(), NewProcessHandle.into()]));
        out
    }

    pub fn ZwGetNextThread(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, ThreadHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, HandleAttributes: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, NewThreadHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwGetNextThread", vec![ProcessHandle.into(), ThreadHandle.into(), DesiredAccess.into(), HandleAttributes.into(), Flags.into(), NewThreadHandle.into()]));
        out
    }

    pub fn ZwGetNlsSectionPtr(&mut self, SectionType: impl Into<IRegOrImm<u64>>, SectionData: impl Into<IRegOrImm<u64>>, ContextData: impl Into<IRegOrImm<u64>>, SectionPointer: impl Into<IRegOrImm<u64>>, SectionSize: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwGetNlsSectionPtr", vec![SectionType.into(), SectionData.into(), ContextData.into(), SectionPointer.into(), SectionSize.into()]));
        out
    }

    pub fn ZwGetNotificationResourceManager(&mut self, ResourceManagerHandle: impl Into<IRegOrImm<u64>>, TransactionNotification: impl Into<IRegOrImm<u64>>, NotificationLength: impl Into<IRegOrImm<u64>>, Timeout: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>, Asynchronous: impl Into<IRegOrImm<u64>>, AsynchronousContext: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwGetNotificationResourceManager", vec![ResourceManagerHandle.into(), TransactionNotification.into(), NotificationLength.into(), Timeout.into(), ReturnLength.into(), Asynchronous.into(), AsynchronousContext.into()]));
        out
    }

    pub fn ZwGetPlugPlayEvent(&mut self, EventHandle: impl Into<IRegOrImm<u64>>, Context: impl Into<IRegOrImm<u64>>, EventBlock: impl Into<IRegOrImm<u64>>, EventBufferSize: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwGetPlugPlayEvent", vec![EventHandle.into(), Context.into(), EventBlock.into(), EventBufferSize.into()]));
        out
    }

    pub fn ZwGetWriteWatch(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, BaseAddress: impl Into<IRegOrImm<u64>>, RegionSize: impl Into<IRegOrImm<u64>>, UserAddressArray: impl Into<IRegOrImm<u64>>, EntriesInUserAddressArray: impl Into<IRegOrImm<u64>>, Granularity: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwGetWriteWatch", vec![ProcessHandle.into(), Flags.into(), BaseAddress.into(), RegionSize.into(), UserAddressArray.into(), EntriesInUserAddressArray.into(), Granularity.into()]));
        out
    }

    pub fn ZwImpersonateAnonymousToken(&mut self, ThreadHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwImpersonateAnonymousToken", vec![ThreadHandle.into()]));
        out
    }

    pub fn ZwImpersonateClientOfPort(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, Message: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwImpersonateClientOfPort", vec![PortHandle.into(), Message.into()]));
        out
    }

    pub fn ZwImpersonateThread(&mut self, ServerThreadHandle: impl Into<IRegOrImm<u64>>, ClientThreadHandle: impl Into<IRegOrImm<u64>>, SecurityQos: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwImpersonateThread", vec![ServerThreadHandle.into(), ClientThreadHandle.into(), SecurityQos.into()]));
        out
    }

    pub fn ZwInitializeEnclave(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, BaseAddress: impl Into<IRegOrImm<u64>>, EnclaveInformation: impl Into<IRegOrImm<u64>>, EnclaveInformationLength: impl Into<IRegOrImm<u64>>, EnclaveError: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwInitializeEnclave", vec![ProcessHandle.into(), BaseAddress.into(), EnclaveInformation.into(), EnclaveInformationLength.into(), EnclaveError.into()]));
        out
    }

    pub fn ZwInitializeNlsFiles(&mut self, BaseAddress: impl Into<IRegOrImm<u64>>, DefaultLocaleId: impl Into<IRegOrImm<u64>>, DefaultCasingTableSize: impl Into<IRegOrImm<u64>>, CurrentNLSVersion: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwInitializeNlsFiles", vec![BaseAddress.into(), DefaultLocaleId.into(), DefaultCasingTableSize.into(), CurrentNLSVersion.into()]));
        out
    }

    pub fn ZwInitializeRegistry(&mut self, BootCondition: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwInitializeRegistry", vec![BootCondition.into()]));
        out
    }

    pub fn ZwInitiatePowerAction(&mut self, SystemAction: impl Into<IRegOrImm<u64>>, LightestSystemState: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, Asynchronous: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwInitiatePowerAction", vec![SystemAction.into(), LightestSystemState.into(), Flags.into(), Asynchronous.into()]));
        out
    }

    pub fn ZwIsProcessInJob(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, JobHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwIsProcessInJob", vec![ProcessHandle.into(), JobHandle.into()]));
        out
    }

    pub fn ZwIsSystemResumeAutomatic(&mut self, ) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwIsSystemResumeAutomatic", vec![]));
        out
    }

    pub fn ZwIsUILanguageComitted(&mut self, ) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwIsUILanguageComitted", vec![]));
        out
    }

    pub fn ZwListenPort(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, ConnectionRequest: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwListenPort", vec![PortHandle.into(), ConnectionRequest.into()]));
        out
    }

    pub fn ZwLoadDriver(&mut self, DriverServiceName: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwLoadDriver", vec![DriverServiceName.into()]));
        out
    }

    pub fn ZwLoadEnclaveData(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, BaseAddress: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>, BufferSize: impl Into<IRegOrImm<u64>>, Protect: impl Into<IRegOrImm<u64>>, PageInformation: impl Into<IRegOrImm<u64>>, PageInformationLength: impl Into<IRegOrImm<u64>>, NumberOfBytesWritten: impl Into<IRegOrImm<u64>>, EnclaveError: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwLoadEnclaveData", vec![ProcessHandle.into(), BaseAddress.into(), Buffer.into(), BufferSize.into(), Protect.into(), PageInformation.into(), PageInformationLength.into(), NumberOfBytesWritten.into(), EnclaveError.into()]));
        out
    }

    pub fn ZwLoadKey(&mut self, TargetKey: impl Into<IRegOrImm<u64>>, SourceFile: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwLoadKey", vec![TargetKey.into(), SourceFile.into()]));
        out
    }

    pub fn ZwLoadKey2(&mut self, TargetKey: impl Into<IRegOrImm<u64>>, SourceFile: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwLoadKey2", vec![TargetKey.into(), SourceFile.into(), Flags.into()]));
        out
    }

    pub fn ZwLoadKey3(&mut self, TargetKey: impl Into<IRegOrImm<u64>>, SourceFile: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, ExtendedParameters: impl Into<IRegOrImm<u64>>, ExtendedParameterCount: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, RootHandle: impl Into<IRegOrImm<u64>>, Reserved: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwLoadKey3", vec![TargetKey.into(), SourceFile.into(), Flags.into(), ExtendedParameters.into(), ExtendedParameterCount.into(), DesiredAccess.into(), RootHandle.into(), Reserved.into()]));
        out
    }

    pub fn ZwLoadKeyEx(&mut self, TargetKey: impl Into<IRegOrImm<u64>>, SourceFile: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, TrustClassKey: impl Into<IRegOrImm<u64>>, Event: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, RootHandle: impl Into<IRegOrImm<u64>>, Reserved: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwLoadKeyEx", vec![TargetKey.into(), SourceFile.into(), Flags.into(), TrustClassKey.into(), Event.into(), DesiredAccess.into(), RootHandle.into(), Reserved.into()]));
        out
    }

    pub fn ZwLockFile(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, Event: impl Into<IRegOrImm<u64>>, ApcRoutine: impl Into<IRegOrImm<u64>>, ApcContext: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, ByteOffset: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, Key: impl Into<IRegOrImm<u64>>, FailImmediately: impl Into<IRegOrImm<u64>>, ExclusiveLock: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwLockFile", vec![FileHandle.into(), Event.into(), ApcRoutine.into(), ApcContext.into(), IoStatusBlock.into(), ByteOffset.into(), Length.into(), Key.into(), FailImmediately.into(), ExclusiveLock.into()]));
        out
    }

    pub fn ZwLockProductActivationKeys(&mut self, pPrivateVer: impl Into<IRegOrImm<u64>>, pSafeMode: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwLockProductActivationKeys", vec![pPrivateVer.into(), pSafeMode.into()]));
        out
    }

    pub fn ZwLockRegistryKey(&mut self, KeyHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwLockRegistryKey", vec![KeyHandle.into()]));
        out
    }

    pub fn ZwLockVirtualMemory(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, BaseAddress: impl Into<IRegOrImm<u64>>, RegionSize: impl Into<IRegOrImm<u64>>, MapType: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwLockVirtualMemory", vec![ProcessHandle.into(), BaseAddress.into(), RegionSize.into(), MapType.into()]));
        out
    }

    pub fn ZwMakePermanentObject(&mut self, Handle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwMakePermanentObject", vec![Handle.into()]));
        out
    }

    pub fn ZwMakeTemporaryObject(&mut self, Handle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwMakeTemporaryObject", vec![Handle.into()]));
        out
    }

    pub fn ZwManagePartition(&mut self, TargetHandle: impl Into<IRegOrImm<u64>>, SourceHandle: impl Into<IRegOrImm<u64>>, PartitionInformationClass: impl Into<IRegOrImm<u64>>, PartitionInformation: impl Into<IRegOrImm<u64>>, PartitionInformationLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwManagePartition", vec![TargetHandle.into(), SourceHandle.into(), PartitionInformationClass.into(), PartitionInformation.into(), PartitionInformationLength.into()]));
        out
    }

    pub fn ZwMapCMFModule(&mut self, What: impl Into<IRegOrImm<u64>>, Index: impl Into<IRegOrImm<u64>>, CacheIndexOut: impl Into<IRegOrImm<u64>>, CacheFlagsOut: impl Into<IRegOrImm<u64>>, ViewSizeOut: impl Into<IRegOrImm<u64>>, BaseAddress: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwMapCMFModule", vec![What.into(), Index.into(), CacheIndexOut.into(), CacheFlagsOut.into(), ViewSizeOut.into(), BaseAddress.into()]));
        out
    }

    pub fn ZwMapUserPhysicalPages(&mut self, VirtualAddress: impl Into<IRegOrImm<u64>>, NumberOfPages: impl Into<IRegOrImm<u64>>, UserPfnArray: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwMapUserPhysicalPages", vec![VirtualAddress.into(), NumberOfPages.into(), UserPfnArray.into()]));
        out
    }

    pub fn ZwMapUserPhysicalPagesScatter(&mut self, VirtualAddresses: impl Into<IRegOrImm<u64>>, NumberOfPages: impl Into<IRegOrImm<u64>>, UserPfnArray: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwMapUserPhysicalPagesScatter", vec![VirtualAddresses.into(), NumberOfPages.into(), UserPfnArray.into()]));
        out
    }

    pub fn ZwMapViewOfSection(&mut self, SectionHandle: impl Into<IRegOrImm<u64>>, ProcessHandle: impl Into<IRegOrImm<u64>>, BaseAddress: impl Into<IRegOrImm<u64>>, ZeroBits: impl Into<IRegOrImm<u64>>, CommitSize: impl Into<IRegOrImm<u64>>, SectionOffset: impl Into<IRegOrImm<u64>>, ViewSize: impl Into<IRegOrImm<u64>>, InheritDisposition: impl Into<IRegOrImm<u64>>, AllocationType: impl Into<IRegOrImm<u64>>, Win32Protect: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwMapViewOfSection", vec![SectionHandle.into(), ProcessHandle.into(), BaseAddress.into(), ZeroBits.into(), CommitSize.into(), SectionOffset.into(), ViewSize.into(), InheritDisposition.into(), AllocationType.into(), Win32Protect.into()]));
        out
    }

    pub fn ZwMapViewOfSectionEx(&mut self, SectionHandle: impl Into<IRegOrImm<u64>>, ProcessHandle: impl Into<IRegOrImm<u64>>, BaseAddress: impl Into<IRegOrImm<u64>>, SectionOffset: impl Into<IRegOrImm<u64>>, ViewSize: impl Into<IRegOrImm<u64>>, AllocationType: impl Into<IRegOrImm<u64>>, Win32Protect: impl Into<IRegOrImm<u64>>, ExtendedParameters: impl Into<IRegOrImm<u64>>, ExtendedParameterCount: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwMapViewOfSectionEx", vec![SectionHandle.into(), ProcessHandle.into(), BaseAddress.into(), SectionOffset.into(), ViewSize.into(), AllocationType.into(), Win32Protect.into(), ExtendedParameters.into(), ExtendedParameterCount.into()]));
        out
    }

    pub fn ZwModifyBootEntry(&mut self, BootEntry: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwModifyBootEntry", vec![BootEntry.into()]));
        out
    }

    pub fn ZwModifyDriverEntry(&mut self, DriverEntry: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwModifyDriverEntry", vec![DriverEntry.into()]));
        out
    }

    pub fn ZwNotifyChangeDirectoryFile(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, Event: impl Into<IRegOrImm<u64>>, ApcRoutine: impl Into<IRegOrImm<u64>>, ApcContext: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, CompletionFilter: impl Into<IRegOrImm<u64>>, WatchTree: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwNotifyChangeDirectoryFile", vec![FileHandle.into(), Event.into(), ApcRoutine.into(), ApcContext.into(), IoStatusBlock.into(), Buffer.into(), Length.into(), CompletionFilter.into(), WatchTree.into()]));
        out
    }

    pub fn ZwNotifyChangeDirectoryFileEx(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, Event: impl Into<IRegOrImm<u64>>, ApcRoutine: impl Into<IRegOrImm<u64>>, ApcContext: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, CompletionFilter: impl Into<IRegOrImm<u64>>, WatchTree: impl Into<IRegOrImm<u64>>, DirectoryNotifyInformationClass: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwNotifyChangeDirectoryFileEx", vec![FileHandle.into(), Event.into(), ApcRoutine.into(), ApcContext.into(), IoStatusBlock.into(), Buffer.into(), Length.into(), CompletionFilter.into(), WatchTree.into(), DirectoryNotifyInformationClass.into()]));
        out
    }

    pub fn ZwNotifyChangeKey(&mut self, KeyHandle: impl Into<IRegOrImm<u64>>, Event: impl Into<IRegOrImm<u64>>, ApcRoutine: impl Into<IRegOrImm<u64>>, ApcContext: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, CompletionFilter: impl Into<IRegOrImm<u64>>, WatchTree: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>, BufferSize: impl Into<IRegOrImm<u64>>, Asynchronous: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwNotifyChangeKey", vec![KeyHandle.into(), Event.into(), ApcRoutine.into(), ApcContext.into(), IoStatusBlock.into(), CompletionFilter.into(), WatchTree.into(), Buffer.into(), BufferSize.into(), Asynchronous.into()]));
        out
    }

    pub fn ZwNotifyChangeMultipleKeys(&mut self, MasterKeyHandle: impl Into<IRegOrImm<u64>>, Count: impl Into<IRegOrImm<u64>>, SubordinateObjects: impl Into<IRegOrImm<u64>>, Event: impl Into<IRegOrImm<u64>>, ApcRoutine: impl Into<IRegOrImm<u64>>, ApcContext: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, CompletionFilter: impl Into<IRegOrImm<u64>>, WatchTree: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>, BufferSize: impl Into<IRegOrImm<u64>>, Asynchronous: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwNotifyChangeMultipleKeys", vec![MasterKeyHandle.into(), Count.into(), SubordinateObjects.into(), Event.into(), ApcRoutine.into(), ApcContext.into(), IoStatusBlock.into(), CompletionFilter.into(), WatchTree.into(), Buffer.into(), BufferSize.into(), Asynchronous.into()]));
        out
    }

    pub fn ZwNotifyChangeSession(&mut self, SessionHandle: impl Into<IRegOrImm<u64>>, ChangeSequenceNumber: impl Into<IRegOrImm<u64>>, ChangeTimeStamp: impl Into<IRegOrImm<u64>>, Event: impl Into<IRegOrImm<u64>>, NewState: impl Into<IRegOrImm<u64>>, PreviousState: impl Into<IRegOrImm<u64>>, Payload: impl Into<IRegOrImm<u64>>, PayloadSize: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwNotifyChangeSession", vec![SessionHandle.into(), ChangeSequenceNumber.into(), ChangeTimeStamp.into(), Event.into(), NewState.into(), PreviousState.into(), Payload.into(), PayloadSize.into()]));
        out
    }

    pub fn ZwOpenDirectoryObject(&mut self, DirectoryHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenDirectoryObject", vec![DirectoryHandle.into(), DesiredAccess.into(), ObjectAttributes.into()]));
        out
    }

    pub fn ZwOpenEnlistment(&mut self, EnlistmentHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ResourceManagerHandle: impl Into<IRegOrImm<u64>>, EnlistmentGuid: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenEnlistment", vec![EnlistmentHandle.into(), DesiredAccess.into(), ResourceManagerHandle.into(), EnlistmentGuid.into(), ObjectAttributes.into()]));
        out
    }

    pub fn ZwOpenEvent(&mut self, EventHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenEvent", vec![EventHandle.into(), DesiredAccess.into(), ObjectAttributes.into()]));
        out
    }

    pub fn ZwOpenEventPair(&mut self, EventPairHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenEventPair", vec![EventPairHandle.into(), DesiredAccess.into(), ObjectAttributes.into()]));
        out
    }

    pub fn ZwOpenFile(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, ShareAccess: impl Into<IRegOrImm<u64>>, OpenOptions: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenFile", vec![FileHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), IoStatusBlock.into(), ShareAccess.into(), OpenOptions.into()]));
        out
    }

    pub fn ZwOpenIoCompletion(&mut self, IoCompletionHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenIoCompletion", vec![IoCompletionHandle.into(), DesiredAccess.into(), ObjectAttributes.into()]));
        out
    }

    pub fn ZwOpenJobObject(&mut self, JobHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenJobObject", vec![JobHandle.into(), DesiredAccess.into(), ObjectAttributes.into()]));
        out
    }

    pub fn ZwOpenKey(&mut self, KeyHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenKey", vec![KeyHandle.into(), DesiredAccess.into(), ObjectAttributes.into()]));
        out
    }

    pub fn ZwOpenKeyedEvent(&mut self, KeyedEventHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenKeyedEvent", vec![KeyedEventHandle.into(), DesiredAccess.into(), ObjectAttributes.into()]));
        out
    }

    pub fn ZwOpenKeyEx(&mut self, KeyHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, OpenOptions: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenKeyEx", vec![KeyHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), OpenOptions.into()]));
        out
    }

    pub fn ZwOpenKeyTransacted(&mut self, KeyHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, TransactionHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenKeyTransacted", vec![KeyHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), TransactionHandle.into()]));
        out
    }

    pub fn ZwOpenKeyTransactedEx(&mut self, KeyHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, OpenOptions: impl Into<IRegOrImm<u64>>, TransactionHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenKeyTransactedEx", vec![KeyHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), OpenOptions.into(), TransactionHandle.into()]));
        out
    }

    pub fn ZwOpenMutant(&mut self, MutantHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenMutant", vec![MutantHandle.into(), DesiredAccess.into(), ObjectAttributes.into()]));
        out
    }

    pub fn ZwOpenObjectAuditAlarm(&mut self, SubsystemName: impl Into<IRegOrImm<u64>>, HandleId: impl Into<IRegOrImm<u64>>, ObjectTypeName: impl Into<IRegOrImm<u64>>, ObjectName: impl Into<IRegOrImm<u64>>, SecurityDescriptor: impl Into<IRegOrImm<u64>>, ClientToken: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, GrantedAccess: impl Into<IRegOrImm<u64>>, Privileges: impl Into<IRegOrImm<u64>>, ObjectCreation: impl Into<IRegOrImm<u64>>, AccessGranted: impl Into<IRegOrImm<u64>>, GenerateOnClose: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenObjectAuditAlarm", vec![SubsystemName.into(), HandleId.into(), ObjectTypeName.into(), ObjectName.into(), SecurityDescriptor.into(), ClientToken.into(), DesiredAccess.into(), GrantedAccess.into(), Privileges.into(), ObjectCreation.into(), AccessGranted.into(), GenerateOnClose.into()]));
        out
    }

    pub fn ZwOpenPartition(&mut self, PartitionHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenPartition", vec![PartitionHandle.into(), DesiredAccess.into(), ObjectAttributes.into()]));
        out
    }

    pub fn ZwOpenPrivateNamespace(&mut self, NamespaceHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, BoundaryDescriptor: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenPrivateNamespace", vec![NamespaceHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), BoundaryDescriptor.into()]));
        out
    }

    pub fn ZwOpenProcess(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, ClientId: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenProcess", vec![ProcessHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), ClientId.into()]));
        out
    }

    pub fn ZwOpenProcessToken(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, TokenHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenProcessToken", vec![ProcessHandle.into(), DesiredAccess.into(), TokenHandle.into()]));
        out
    }

    pub fn ZwOpenProcessTokenEx(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, HandleAttributes: impl Into<IRegOrImm<u64>>, TokenHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenProcessTokenEx", vec![ProcessHandle.into(), DesiredAccess.into(), HandleAttributes.into(), TokenHandle.into()]));
        out
    }

    pub fn ZwOpenResourceManager(&mut self, ResourceManagerHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, TmHandle: impl Into<IRegOrImm<u64>>, ResourceManagerGuid: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenResourceManager", vec![ResourceManagerHandle.into(), DesiredAccess.into(), TmHandle.into(), ResourceManagerGuid.into(), ObjectAttributes.into()]));
        out
    }

    pub fn ZwOpenSection(&mut self, SectionHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenSection", vec![SectionHandle.into(), DesiredAccess.into(), ObjectAttributes.into()]));
        out
    }

    pub fn ZwOpenSemaphore(&mut self, SemaphoreHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenSemaphore", vec![SemaphoreHandle.into(), DesiredAccess.into(), ObjectAttributes.into()]));
        out
    }

    pub fn ZwOpenSession(&mut self, SessionHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenSession", vec![SessionHandle.into(), DesiredAccess.into(), ObjectAttributes.into()]));
        out
    }

    pub fn ZwOpenSymbolicLinkObject(&mut self, LinkHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenSymbolicLinkObject", vec![LinkHandle.into(), DesiredAccess.into(), ObjectAttributes.into()]));
        out
    }

    pub fn ZwOpenThread(&mut self, ThreadHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, ClientId: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenThread", vec![ThreadHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), ClientId.into()]));
        out
    }

    pub fn ZwOpenThreadToken(&mut self, ThreadHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, OpenAsSelf: impl Into<IRegOrImm<u64>>, TokenHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenThreadToken", vec![ThreadHandle.into(), DesiredAccess.into(), OpenAsSelf.into(), TokenHandle.into()]));
        out
    }

    pub fn ZwOpenThreadTokenEx(&mut self, ThreadHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, OpenAsSelf: impl Into<IRegOrImm<u64>>, HandleAttributes: impl Into<IRegOrImm<u64>>, TokenHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenThreadTokenEx", vec![ThreadHandle.into(), DesiredAccess.into(), OpenAsSelf.into(), HandleAttributes.into(), TokenHandle.into()]));
        out
    }

    pub fn ZwOpenTimer(&mut self, TimerHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenTimer", vec![TimerHandle.into(), DesiredAccess.into(), ObjectAttributes.into()]));
        out
    }

    pub fn ZwOpenTransaction(&mut self, TransactionHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, Uow: impl Into<IRegOrImm<u64>>, TmHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenTransaction", vec![TransactionHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), Uow.into(), TmHandle.into()]));
        out
    }

    pub fn ZwOpenTransactionManager(&mut self, TmHandle: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, ObjectAttributes: impl Into<IRegOrImm<u64>>, LogFileName: impl Into<IRegOrImm<u64>>, TmIdentity: impl Into<IRegOrImm<u64>>, OpenOptions: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwOpenTransactionManager", vec![TmHandle.into(), DesiredAccess.into(), ObjectAttributes.into(), LogFileName.into(), TmIdentity.into(), OpenOptions.into()]));
        out
    }

    pub fn ZwPlugPlayControl(&mut self, PnPControlClass: impl Into<IRegOrImm<u64>>, PnPControlData: impl Into<IRegOrImm<u64>>, PnPControlDataLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwPlugPlayControl", vec![PnPControlClass.into(), PnPControlData.into(), PnPControlDataLength.into()]));
        out
    }

    pub fn ZwPowerInformation(&mut self, InformationLevel: impl Into<IRegOrImm<u64>>, InputBuffer: impl Into<IRegOrImm<u64>>, InputBufferLength: impl Into<IRegOrImm<u64>>, OutputBuffer: impl Into<IRegOrImm<u64>>, OutputBufferLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwPowerInformation", vec![InformationLevel.into(), InputBuffer.into(), InputBufferLength.into(), OutputBuffer.into(), OutputBufferLength.into()]));
        out
    }

    pub fn ZwPrepareComplete(&mut self, EnlistmentHandle: impl Into<IRegOrImm<u64>>, TmVirtualClock: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwPrepareComplete", vec![EnlistmentHandle.into(), TmVirtualClock.into()]));
        out
    }

    pub fn ZwPrepareEnlistment(&mut self, EnlistmentHandle: impl Into<IRegOrImm<u64>>, TmVirtualClock: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwPrepareEnlistment", vec![EnlistmentHandle.into(), TmVirtualClock.into()]));
        out
    }

    pub fn ZwPrePrepareComplete(&mut self, EnlistmentHandle: impl Into<IRegOrImm<u64>>, TmVirtualClock: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwPrePrepareComplete", vec![EnlistmentHandle.into(), TmVirtualClock.into()]));
        out
    }

    pub fn ZwPrePrepareEnlistment(&mut self, EnlistmentHandle: impl Into<IRegOrImm<u64>>, TmVirtualClock: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwPrePrepareEnlistment", vec![EnlistmentHandle.into(), TmVirtualClock.into()]));
        out
    }

    pub fn ZwPrivilegeCheck(&mut self, ClientToken: impl Into<IRegOrImm<u64>>, RequiredPrivileges: impl Into<IRegOrImm<u64>>, Result: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwPrivilegeCheck", vec![ClientToken.into(), RequiredPrivileges.into(), Result.into()]));
        out
    }

    pub fn ZwPrivilegedServiceAuditAlarm(&mut self, SubsystemName: impl Into<IRegOrImm<u64>>, ServiceName: impl Into<IRegOrImm<u64>>, ClientToken: impl Into<IRegOrImm<u64>>, Privileges: impl Into<IRegOrImm<u64>>, AccessGranted: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwPrivilegedServiceAuditAlarm", vec![SubsystemName.into(), ServiceName.into(), ClientToken.into(), Privileges.into(), AccessGranted.into()]));
        out
    }

    pub fn ZwPrivilegeObjectAuditAlarm(&mut self, SubsystemName: impl Into<IRegOrImm<u64>>, HandleId: impl Into<IRegOrImm<u64>>, ClientToken: impl Into<IRegOrImm<u64>>, DesiredAccess: impl Into<IRegOrImm<u64>>, Privileges: impl Into<IRegOrImm<u64>>, AccessGranted: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwPrivilegeObjectAuditAlarm", vec![SubsystemName.into(), HandleId.into(), ClientToken.into(), DesiredAccess.into(), Privileges.into(), AccessGranted.into()]));
        out
    }

    pub fn ZwPropagationComplete(&mut self, ResourceManagerHandle: impl Into<IRegOrImm<u64>>, RequestCookie: impl Into<IRegOrImm<u64>>, BufferLength: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwPropagationComplete", vec![ResourceManagerHandle.into(), RequestCookie.into(), BufferLength.into(), Buffer.into()]));
        out
    }

    pub fn ZwPropagationFailed(&mut self, ResourceManagerHandle: impl Into<IRegOrImm<u64>>, RequestCookie: impl Into<IRegOrImm<u64>>, PropStatus: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwPropagationFailed", vec![ResourceManagerHandle.into(), RequestCookie.into(), PropStatus.into()]));
        out
    }

    pub fn ZwProtectVirtualMemory(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, BaseAddress: impl Into<IRegOrImm<u64>>, RegionSize: impl Into<IRegOrImm<u64>>, NewProtect: impl Into<IRegOrImm<u64>>, OldProtect: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwProtectVirtualMemory", vec![ProcessHandle.into(), BaseAddress.into(), RegionSize.into(), NewProtect.into(), OldProtect.into()]));
        out
    }

    pub fn ZwPssCaptureVaSpaceBulk(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, BaseAddress: impl Into<IRegOrImm<u64>>, BulkInformation: impl Into<IRegOrImm<u64>>, BulkInformationLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwPssCaptureVaSpaceBulk", vec![ProcessHandle.into(), BaseAddress.into(), BulkInformation.into(), BulkInformationLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwPulseEvent(&mut self, EventHandle: impl Into<IRegOrImm<u64>>, PreviousState: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwPulseEvent", vec![EventHandle.into(), PreviousState.into()]));
        out
    }

    pub fn ZwQueryAttributesFile(&mut self, ObjectAttributes: impl Into<IRegOrImm<u64>>, FileInformation: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryAttributesFile", vec![ObjectAttributes.into(), FileInformation.into()]));
        out
    }

    pub fn ZwQueryAuxiliaryCounterFrequency(&mut self, AuxiliaryCounterFrequency: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryAuxiliaryCounterFrequency", vec![AuxiliaryCounterFrequency.into()]));
        out
    }

    pub fn ZwQueryBootEntryOrder(&mut self, Ids: impl Into<IRegOrImm<u64>>, Count: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryBootEntryOrder", vec![Ids.into(), Count.into()]));
        out
    }

    pub fn ZwQueryBootOptions(&mut self, BootOptions: impl Into<IRegOrImm<u64>>, BootOptionsLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryBootOptions", vec![BootOptions.into(), BootOptionsLength.into()]));
        out
    }

    pub fn ZwQueryDebugFilterState(&mut self, ComponentId: impl Into<IRegOrImm<u64>>, Level: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryDebugFilterState", vec![ComponentId.into(), Level.into()]));
        out
    }

    pub fn ZwQueryDefaultLocale(&mut self, UserProfile: impl Into<IRegOrImm<u64>>, DefaultLocaleId: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryDefaultLocale", vec![UserProfile.into(), DefaultLocaleId.into()]));
        out
    }

    pub fn ZwQueryDefaultUILanguage(&mut self, DefaultUILanguageId: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryDefaultUILanguage", vec![DefaultUILanguageId.into()]));
        out
    }

    pub fn ZwQueryDirectoryFile(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, Event: impl Into<IRegOrImm<u64>>, ApcRoutine: impl Into<IRegOrImm<u64>>, ApcContext: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, FileInformation: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, FileInformationClass: impl Into<IRegOrImm<u64>>, ReturnSingleEntry: impl Into<IRegOrImm<u64>>, FileName: impl Into<IRegOrImm<u64>>, RestartScan: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryDirectoryFile", vec![FileHandle.into(), Event.into(), ApcRoutine.into(), ApcContext.into(), IoStatusBlock.into(), FileInformation.into(), Length.into(), FileInformationClass.into(), ReturnSingleEntry.into(), FileName.into(), RestartScan.into()]));
        out
    }

    pub fn ZwQueryDirectoryFileEx(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, Event: impl Into<IRegOrImm<u64>>, ApcRoutine: impl Into<IRegOrImm<u64>>, ApcContext: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, FileInformation: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, FileInformationClass: impl Into<IRegOrImm<u64>>, QueryFlags: impl Into<IRegOrImm<u64>>, FileName: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryDirectoryFileEx", vec![FileHandle.into(), Event.into(), ApcRoutine.into(), ApcContext.into(), IoStatusBlock.into(), FileInformation.into(), Length.into(), FileInformationClass.into(), QueryFlags.into(), FileName.into()]));
        out
    }

    pub fn ZwQueryDirectoryObject(&mut self, DirectoryHandle: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, ReturnSingleEntry: impl Into<IRegOrImm<u64>>, RestartScan: impl Into<IRegOrImm<u64>>, Context: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryDirectoryObject", vec![DirectoryHandle.into(), Buffer.into(), Length.into(), ReturnSingleEntry.into(), RestartScan.into(), Context.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwQueryDriverEntryOrder(&mut self, Ids: impl Into<IRegOrImm<u64>>, Count: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryDriverEntryOrder", vec![Ids.into(), Count.into()]));
        out
    }

    pub fn ZwQueryEaFile(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, ReturnSingleEntry: impl Into<IRegOrImm<u64>>, EaList: impl Into<IRegOrImm<u64>>, EaListLength: impl Into<IRegOrImm<u64>>, EaIndex: impl Into<IRegOrImm<u64>>, RestartScan: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryEaFile", vec![FileHandle.into(), IoStatusBlock.into(), Buffer.into(), Length.into(), ReturnSingleEntry.into(), EaList.into(), EaListLength.into(), EaIndex.into(), RestartScan.into()]));
        out
    }

    pub fn ZwQueryEvent(&mut self, EventHandle: impl Into<IRegOrImm<u64>>, EventInformationClass: impl Into<IRegOrImm<u64>>, EventInformation: impl Into<IRegOrImm<u64>>, EventInformationLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryEvent", vec![EventHandle.into(), EventInformationClass.into(), EventInformation.into(), EventInformationLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwQueryFullAttributesFile(&mut self, ObjectAttributes: impl Into<IRegOrImm<u64>>, FileInformation: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryFullAttributesFile", vec![ObjectAttributes.into(), FileInformation.into()]));
        out
    }

    pub fn ZwQueryInformationAtom(&mut self, Atom: impl Into<IRegOrImm<u64>>, AtomInformationClass: impl Into<IRegOrImm<u64>>, AtomInformation: impl Into<IRegOrImm<u64>>, AtomInformationLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryInformationAtom", vec![Atom.into(), AtomInformationClass.into(), AtomInformation.into(), AtomInformationLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwQueryInformationByName(&mut self, ObjectAttributes: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, FileInformation: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, FileInformationClass: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryInformationByName", vec![ObjectAttributes.into(), IoStatusBlock.into(), FileInformation.into(), Length.into(), FileInformationClass.into()]));
        out
    }

    pub fn ZwQueryInformationEnlistment(&mut self, EnlistmentHandle: impl Into<IRegOrImm<u64>>, EnlistmentInformationClass: impl Into<IRegOrImm<u64>>, EnlistmentInformation: impl Into<IRegOrImm<u64>>, EnlistmentInformationLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryInformationEnlistment", vec![EnlistmentHandle.into(), EnlistmentInformationClass.into(), EnlistmentInformation.into(), EnlistmentInformationLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwQueryInformationFile(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, FileInformation: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, FileInformationClass: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryInformationFile", vec![FileHandle.into(), IoStatusBlock.into(), FileInformation.into(), Length.into(), FileInformationClass.into()]));
        out
    }

    pub fn ZwQueryInformationJobObject(&mut self, JobHandle: impl Into<IRegOrImm<u64>>, JobObjectInformationClass: impl Into<IRegOrImm<u64>>, JobObjectInformation: impl Into<IRegOrImm<u64>>, JobObjectInformationLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryInformationJobObject", vec![JobHandle.into(), JobObjectInformationClass.into(), JobObjectInformation.into(), JobObjectInformationLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwQueryInformationPort(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, PortInformationClass: impl Into<IRegOrImm<u64>>, PortInformation: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryInformationPort", vec![PortHandle.into(), PortInformationClass.into(), PortInformation.into(), Length.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwQueryInformationProcess(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, ProcessInformationClass: impl Into<IRegOrImm<u64>>, ProcessInformation: impl Into<IRegOrImm<u64>>, ProcessInformationLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryInformationProcess", vec![ProcessHandle.into(), ProcessInformationClass.into(), ProcessInformation.into(), ProcessInformationLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwQueryInformationResourceManager(&mut self, ResourceManagerHandle: impl Into<IRegOrImm<u64>>, ResourceManagerInformationClass: impl Into<IRegOrImm<u64>>, ResourceManagerInformation: impl Into<IRegOrImm<u64>>, ResourceManagerInformationLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryInformationResourceManager", vec![ResourceManagerHandle.into(), ResourceManagerInformationClass.into(), ResourceManagerInformation.into(), ResourceManagerInformationLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwQueryInformationThread(&mut self, ThreadHandle: impl Into<IRegOrImm<u64>>, ThreadInformationClass: impl Into<IRegOrImm<u64>>, ThreadInformation: impl Into<IRegOrImm<u64>>, ThreadInformationLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryInformationThread", vec![ThreadHandle.into(), ThreadInformationClass.into(), ThreadInformation.into(), ThreadInformationLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwQueryInformationToken(&mut self, TokenHandle: impl Into<IRegOrImm<u64>>, TokenInformationClass: impl Into<IRegOrImm<u64>>, TokenInformation: impl Into<IRegOrImm<u64>>, TokenInformationLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryInformationToken", vec![TokenHandle.into(), TokenInformationClass.into(), TokenInformation.into(), TokenInformationLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwQueryInformationTransaction(&mut self, TransactionHandle: impl Into<IRegOrImm<u64>>, TransactionInformationClass: impl Into<IRegOrImm<u64>>, TransactionInformation: impl Into<IRegOrImm<u64>>, TransactionInformationLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryInformationTransaction", vec![TransactionHandle.into(), TransactionInformationClass.into(), TransactionInformation.into(), TransactionInformationLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwQueryInformationTransactionManager(&mut self, TransactionManagerHandle: impl Into<IRegOrImm<u64>>, TransactionManagerInformationClass: impl Into<IRegOrImm<u64>>, TransactionManagerInformation: impl Into<IRegOrImm<u64>>, TransactionManagerInformationLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryInformationTransactionManager", vec![TransactionManagerHandle.into(), TransactionManagerInformationClass.into(), TransactionManagerInformation.into(), TransactionManagerInformationLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwQueryInformationWorkerFactory(&mut self, WorkerFactoryHandle: impl Into<IRegOrImm<u64>>, WorkerFactoryInformationClass: impl Into<IRegOrImm<u64>>, WorkerFactoryInformation: impl Into<IRegOrImm<u64>>, WorkerFactoryInformationLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryInformationWorkerFactory", vec![WorkerFactoryHandle.into(), WorkerFactoryInformationClass.into(), WorkerFactoryInformation.into(), WorkerFactoryInformationLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwQueryInstallUILanguage(&mut self, InstallUILanguageId: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryInstallUILanguage", vec![InstallUILanguageId.into()]));
        out
    }

    pub fn ZwQueryIntervalProfile(&mut self, ProfileSource: impl Into<IRegOrImm<u64>>, Interval: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryIntervalProfile", vec![ProfileSource.into(), Interval.into()]));
        out
    }

    pub fn ZwQueryIoCompletion(&mut self, IoCompletionHandle: impl Into<IRegOrImm<u64>>, IoCompletionInformationClass: impl Into<IRegOrImm<u64>>, IoCompletionInformation: impl Into<IRegOrImm<u64>>, IoCompletionInformationLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryIoCompletion", vec![IoCompletionHandle.into(), IoCompletionInformationClass.into(), IoCompletionInformation.into(), IoCompletionInformationLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwQueryIoRingCapabilities(&mut self, IoRingCapabilitiesLength: impl Into<IRegOrImm<u64>>, IoRingCapabilities: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryIoRingCapabilities", vec![IoRingCapabilitiesLength.into(), IoRingCapabilities.into()]));
        out
    }

    pub fn ZwQueryKey(&mut self, KeyHandle: impl Into<IRegOrImm<u64>>, KeyInformationClass: impl Into<IRegOrImm<u64>>, KeyInformation: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, ResultLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryKey", vec![KeyHandle.into(), KeyInformationClass.into(), KeyInformation.into(), Length.into(), ResultLength.into()]));
        out
    }

    pub fn ZwQueryLicenseValue(&mut self, ValueName: impl Into<IRegOrImm<u64>>, Type: impl Into<IRegOrImm<u64>>, Data: impl Into<IRegOrImm<u64>>, DataSize: impl Into<IRegOrImm<u64>>, ResultDataSize: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryLicenseValue", vec![ValueName.into(), Type.into(), Data.into(), DataSize.into(), ResultDataSize.into()]));
        out
    }

    pub fn ZwQueryMultipleValueKey(&mut self, KeyHandle: impl Into<IRegOrImm<u64>>, ValueEntries: impl Into<IRegOrImm<u64>>, EntryCount: impl Into<IRegOrImm<u64>>, ValueBuffer: impl Into<IRegOrImm<u64>>, BufferLength: impl Into<IRegOrImm<u64>>, RequiredBufferLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryMultipleValueKey", vec![KeyHandle.into(), ValueEntries.into(), EntryCount.into(), ValueBuffer.into(), BufferLength.into(), RequiredBufferLength.into()]));
        out
    }

    pub fn ZwQueryMutant(&mut self, MutantHandle: impl Into<IRegOrImm<u64>>, MutantInformationClass: impl Into<IRegOrImm<u64>>, MutantInformation: impl Into<IRegOrImm<u64>>, MutantInformationLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryMutant", vec![MutantHandle.into(), MutantInformationClass.into(), MutantInformation.into(), MutantInformationLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwQueryObject(&mut self, Handle: impl Into<IRegOrImm<u64>>, ObjectInformationClass: impl Into<IRegOrImm<u64>>, ObjectInformation: impl Into<IRegOrImm<u64>>, ObjectInformationLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryObject", vec![Handle.into(), ObjectInformationClass.into(), ObjectInformation.into(), ObjectInformationLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwQueryOpenSubKeys(&mut self, TargetKey: impl Into<IRegOrImm<u64>>, HandleCount: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryOpenSubKeys", vec![TargetKey.into(), HandleCount.into()]));
        out
    }

    pub fn ZwQueryOpenSubKeysEx(&mut self, TargetKey: impl Into<IRegOrImm<u64>>, BufferLength: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>, RequiredSize: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryOpenSubKeysEx", vec![TargetKey.into(), BufferLength.into(), Buffer.into(), RequiredSize.into()]));
        out
    }

    pub fn ZwQueryPerformanceCounter(&mut self, PerformanceCounter: impl Into<IRegOrImm<u64>>, PerformanceFrequency: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryPerformanceCounter", vec![PerformanceCounter.into(), PerformanceFrequency.into()]));
        out
    }

    pub fn ZwQueryPortInformationProcess(&mut self, ) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryPortInformationProcess", vec![]));
        out
    }

    pub fn ZwQueryQuotaInformationFile(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, ReturnSingleEntry: impl Into<IRegOrImm<u64>>, SidList: impl Into<IRegOrImm<u64>>, SidListLength: impl Into<IRegOrImm<u64>>, StartSid: impl Into<IRegOrImm<u64>>, RestartScan: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryQuotaInformationFile", vec![FileHandle.into(), IoStatusBlock.into(), Buffer.into(), Length.into(), ReturnSingleEntry.into(), SidList.into(), SidListLength.into(), StartSid.into(), RestartScan.into()]));
        out
    }

    pub fn ZwQuerySection(&mut self, SectionHandle: impl Into<IRegOrImm<u64>>, SectionInformationClass: impl Into<IRegOrImm<u64>>, SectionInformation: impl Into<IRegOrImm<u64>>, SectionInformationLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQuerySection", vec![SectionHandle.into(), SectionInformationClass.into(), SectionInformation.into(), SectionInformationLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwQuerySecurityAttributesToken(&mut self, TokenHandle: impl Into<IRegOrImm<u64>>, Attributes: impl Into<IRegOrImm<u64>>, NumberOfAttributes: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQuerySecurityAttributesToken", vec![TokenHandle.into(), Attributes.into(), NumberOfAttributes.into(), Buffer.into(), Length.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwQuerySecurityObject(&mut self, Handle: impl Into<IRegOrImm<u64>>, SecurityInformation: impl Into<IRegOrImm<u64>>, SecurityDescriptor: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, LengthNeeded: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQuerySecurityObject", vec![Handle.into(), SecurityInformation.into(), SecurityDescriptor.into(), Length.into(), LengthNeeded.into()]));
        out
    }

    pub fn ZwQuerySemaphore(&mut self, SemaphoreHandle: impl Into<IRegOrImm<u64>>, SemaphoreInformationClass: impl Into<IRegOrImm<u64>>, SemaphoreInformation: impl Into<IRegOrImm<u64>>, SemaphoreInformationLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQuerySemaphore", vec![SemaphoreHandle.into(), SemaphoreInformationClass.into(), SemaphoreInformation.into(), SemaphoreInformationLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwQuerySymbolicLinkObject(&mut self, LinkHandle: impl Into<IRegOrImm<u64>>, LinkTarget: impl Into<IRegOrImm<u64>>, ReturnedLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQuerySymbolicLinkObject", vec![LinkHandle.into(), LinkTarget.into(), ReturnedLength.into()]));
        out
    }

    pub fn ZwQuerySystemEnvironmentValue(&mut self, VariableName: impl Into<IRegOrImm<u64>>, VariableValue: impl Into<IRegOrImm<u64>>, ValueLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQuerySystemEnvironmentValue", vec![VariableName.into(), VariableValue.into(), ValueLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwQuerySystemEnvironmentValueEx(&mut self, VariableName: impl Into<IRegOrImm<u64>>, VendorGuid: impl Into<IRegOrImm<u64>>, Value: impl Into<IRegOrImm<u64>>, ValueLength: impl Into<IRegOrImm<u64>>, Attributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQuerySystemEnvironmentValueEx", vec![VariableName.into(), VendorGuid.into(), Value.into(), ValueLength.into(), Attributes.into()]));
        out
    }

    pub fn ZwQuerySystemInformation(&mut self, SystemInformationClass: impl Into<IRegOrImm<u64>>, SystemInformation: impl Into<IRegOrImm<u64>>, SystemInformationLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQuerySystemInformation", vec![SystemInformationClass.into(), SystemInformation.into(), SystemInformationLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwQuerySystemInformationEx(&mut self, SystemInformationClass: impl Into<IRegOrImm<u64>>, InputBuffer: impl Into<IRegOrImm<u64>>, InputBufferLength: impl Into<IRegOrImm<u64>>, SystemInformation: impl Into<IRegOrImm<u64>>, SystemInformationLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQuerySystemInformationEx", vec![SystemInformationClass.into(), InputBuffer.into(), InputBufferLength.into(), SystemInformation.into(), SystemInformationLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwQuerySystemTime(&mut self, SystemTime: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQuerySystemTime", vec![SystemTime.into()]));
        out
    }

    pub fn ZwQueryTimer(&mut self, TimerHandle: impl Into<IRegOrImm<u64>>, TimerInformationClass: impl Into<IRegOrImm<u64>>, TimerInformation: impl Into<IRegOrImm<u64>>, TimerInformationLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryTimer", vec![TimerHandle.into(), TimerInformationClass.into(), TimerInformation.into(), TimerInformationLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwQueryTimerResolution(&mut self, MaximumTime: impl Into<IRegOrImm<u64>>, MinimumTime: impl Into<IRegOrImm<u64>>, CurrentTime: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryTimerResolution", vec![MaximumTime.into(), MinimumTime.into(), CurrentTime.into()]));
        out
    }

    pub fn ZwQueryValueKey(&mut self, KeyHandle: impl Into<IRegOrImm<u64>>, ValueName: impl Into<IRegOrImm<u64>>, KeyValueInformationClass: impl Into<IRegOrImm<u64>>, KeyValueInformation: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, ResultLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryValueKey", vec![KeyHandle.into(), ValueName.into(), KeyValueInformationClass.into(), KeyValueInformation.into(), Length.into(), ResultLength.into()]));
        out
    }

    pub fn ZwQueryVirtualMemory(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, BaseAddress: impl Into<IRegOrImm<u64>>, MemoryInformationClass: impl Into<IRegOrImm<u64>>, MemoryInformation: impl Into<IRegOrImm<u64>>, MemoryInformationLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryVirtualMemory", vec![ProcessHandle.into(), BaseAddress.into(), MemoryInformationClass.into(), MemoryInformation.into(), MemoryInformationLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwQueryVolumeInformationFile(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, FsInformation: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, FsInformationClass: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryVolumeInformationFile", vec![FileHandle.into(), IoStatusBlock.into(), FsInformation.into(), Length.into(), FsInformationClass.into()]));
        out
    }

    pub fn ZwQueryWnfStateData(&mut self, StateName: impl Into<IRegOrImm<u64>>, TypeId: impl Into<IRegOrImm<u64>>, ExplicitScope: impl Into<IRegOrImm<u64>>, ChangeStamp: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>, BufferSize: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryWnfStateData", vec![StateName.into(), TypeId.into(), ExplicitScope.into(), ChangeStamp.into(), Buffer.into(), BufferSize.into()]));
        out
    }

    pub fn ZwQueryWnfStateNameInformation(&mut self, StateName: impl Into<IRegOrImm<u64>>, NameInfoClass: impl Into<IRegOrImm<u64>>, ExplicitScope: impl Into<IRegOrImm<u64>>, InfoBuffer: impl Into<IRegOrImm<u64>>, InfoBufferSize: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueryWnfStateNameInformation", vec![StateName.into(), NameInfoClass.into(), ExplicitScope.into(), InfoBuffer.into(), InfoBufferSize.into()]));
        out
    }

    pub fn ZwQueueApcThread(&mut self, ThreadHandle: impl Into<IRegOrImm<u64>>, ApcRoutine: impl Into<IRegOrImm<u64>>, ApcArgument1: impl Into<IRegOrImm<u64>>, ApcArgument2: impl Into<IRegOrImm<u64>>, ApcArgument3: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueueApcThread", vec![ThreadHandle.into(), ApcRoutine.into(), ApcArgument1.into(), ApcArgument2.into(), ApcArgument3.into()]));
        out
    }

    pub fn ZwQueueApcThreadEx(&mut self, ThreadHandle: impl Into<IRegOrImm<u64>>, ReserveHandle: impl Into<IRegOrImm<u64>>, ApcRoutine: impl Into<IRegOrImm<u64>>, ApcArgument1: impl Into<IRegOrImm<u64>>, ApcArgument2: impl Into<IRegOrImm<u64>>, ApcArgument3: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueueApcThreadEx", vec![ThreadHandle.into(), ReserveHandle.into(), ApcRoutine.into(), ApcArgument1.into(), ApcArgument2.into(), ApcArgument3.into()]));
        out
    }

    pub fn ZwQueueApcThreadEx2(&mut self, ThreadHandle: impl Into<IRegOrImm<u64>>, ReserveHandle: impl Into<IRegOrImm<u64>>, ApcFlags: impl Into<IRegOrImm<u64>>, ApcRoutine: impl Into<IRegOrImm<u64>>, ApcArgument1: impl Into<IRegOrImm<u64>>, ApcArgument2: impl Into<IRegOrImm<u64>>, ApcArgument3: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwQueueApcThreadEx2", vec![ThreadHandle.into(), ReserveHandle.into(), ApcFlags.into(), ApcRoutine.into(), ApcArgument1.into(), ApcArgument2.into(), ApcArgument3.into()]));
        out
    }

    pub fn ZwRaiseException(&mut self, ExceptionRecord: impl Into<IRegOrImm<u64>>, ContextRecord: impl Into<IRegOrImm<u64>>, FirstChance: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwRaiseException", vec![ExceptionRecord.into(), ContextRecord.into(), FirstChance.into()]));
        out
    }

    pub fn ZwRaiseHardError(&mut self, ErrorStatus: impl Into<IRegOrImm<u64>>, NumberOfParameters: impl Into<IRegOrImm<u64>>, UnicodeStringParameterMask: impl Into<IRegOrImm<u64>>, Parameters: impl Into<IRegOrImm<u64>>, ValidResponseOptions: impl Into<IRegOrImm<u64>>, Response: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwRaiseHardError", vec![ErrorStatus.into(), NumberOfParameters.into(), UnicodeStringParameterMask.into(), Parameters.into(), ValidResponseOptions.into(), Response.into()]));
        out
    }

    pub fn ZwReadFile(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, Event: impl Into<IRegOrImm<u64>>, ApcRoutine: impl Into<IRegOrImm<u64>>, ApcContext: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, ByteOffset: impl Into<IRegOrImm<u64>>, Key: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwReadFile", vec![FileHandle.into(), Event.into(), ApcRoutine.into(), ApcContext.into(), IoStatusBlock.into(), Buffer.into(), Length.into(), ByteOffset.into(), Key.into()]));
        out
    }

    pub fn ZwReadFileScatter(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, Event: impl Into<IRegOrImm<u64>>, ApcRoutine: impl Into<IRegOrImm<u64>>, ApcContext: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, SegmentArray: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, ByteOffset: impl Into<IRegOrImm<u64>>, Key: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwReadFileScatter", vec![FileHandle.into(), Event.into(), ApcRoutine.into(), ApcContext.into(), IoStatusBlock.into(), SegmentArray.into(), Length.into(), ByteOffset.into(), Key.into()]));
        out
    }

    pub fn ZwReadOnlyEnlistment(&mut self, EnlistmentHandle: impl Into<IRegOrImm<u64>>, TmVirtualClock: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwReadOnlyEnlistment", vec![EnlistmentHandle.into(), TmVirtualClock.into()]));
        out
    }

    pub fn ZwReadRequestData(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, Message: impl Into<IRegOrImm<u64>>, DataEntryIndex: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>, BufferSize: impl Into<IRegOrImm<u64>>, NumberOfBytesRead: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwReadRequestData", vec![PortHandle.into(), Message.into(), DataEntryIndex.into(), Buffer.into(), BufferSize.into(), NumberOfBytesRead.into()]));
        out
    }

    pub fn ZwReadVirtualMemory(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, BaseAddress: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>, BufferSize: impl Into<IRegOrImm<u64>>, NumberOfBytesRead: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwReadVirtualMemory", vec![ProcessHandle.into(), BaseAddress.into(), Buffer.into(), BufferSize.into(), NumberOfBytesRead.into()]));
        out
    }

    pub fn ZwReadVirtualMemoryEx(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, BaseAddress: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>, BufferSize: impl Into<IRegOrImm<u64>>, NumberOfBytesRead: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwReadVirtualMemoryEx", vec![ProcessHandle.into(), BaseAddress.into(), Buffer.into(), BufferSize.into(), NumberOfBytesRead.into(), Flags.into()]));
        out
    }

    pub fn ZwRecoverEnlistment(&mut self, EnlistmentHandle: impl Into<IRegOrImm<u64>>, EnlistmentKey: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwRecoverEnlistment", vec![EnlistmentHandle.into(), EnlistmentKey.into()]));
        out
    }

    pub fn ZwRecoverResourceManager(&mut self, ResourceManagerHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwRecoverResourceManager", vec![ResourceManagerHandle.into()]));
        out
    }

    pub fn ZwRecoverTransactionManager(&mut self, TransactionManagerHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwRecoverTransactionManager", vec![TransactionManagerHandle.into()]));
        out
    }

    pub fn ZwRegisterProtocolAddressInformation(&mut self, ResourceManager: impl Into<IRegOrImm<u64>>, ProtocolId: impl Into<IRegOrImm<u64>>, ProtocolInformationSize: impl Into<IRegOrImm<u64>>, ProtocolInformation: impl Into<IRegOrImm<u64>>, CreateOptions: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwRegisterProtocolAddressInformation", vec![ResourceManager.into(), ProtocolId.into(), ProtocolInformationSize.into(), ProtocolInformation.into(), CreateOptions.into()]));
        out
    }

    pub fn ZwRegisterThreadTerminatePort(&mut self, PortHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwRegisterThreadTerminatePort", vec![PortHandle.into()]));
        out
    }

    pub fn ZwReleaseCMFViewOwnership(&mut self, ) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwReleaseCMFViewOwnership", vec![]));
        out
    }

    pub fn ZwReleaseKeyedEvent(&mut self, KeyedEventHandle: impl Into<IRegOrImm<u64>>, KeyValue: impl Into<IRegOrImm<u64>>, Alertable: impl Into<IRegOrImm<u64>>, Timeout: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwReleaseKeyedEvent", vec![KeyedEventHandle.into(), KeyValue.into(), Alertable.into(), Timeout.into()]));
        out
    }

    pub fn ZwReleaseMutant(&mut self, MutantHandle: impl Into<IRegOrImm<u64>>, PreviousCount: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwReleaseMutant", vec![MutantHandle.into(), PreviousCount.into()]));
        out
    }

    pub fn ZwReleaseSemaphore(&mut self, SemaphoreHandle: impl Into<IRegOrImm<u64>>, ReleaseCount: impl Into<IRegOrImm<u64>>, PreviousCount: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwReleaseSemaphore", vec![SemaphoreHandle.into(), ReleaseCount.into(), PreviousCount.into()]));
        out
    }

    pub fn ZwReleaseWorkerFactoryWorker(&mut self, WorkerFactoryHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwReleaseWorkerFactoryWorker", vec![WorkerFactoryHandle.into()]));
        out
    }

    pub fn ZwRemoveIoCompletion(&mut self, IoCompletionHandle: impl Into<IRegOrImm<u64>>, KeyContext: impl Into<IRegOrImm<u64>>, ApcContext: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, Timeout: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwRemoveIoCompletion", vec![IoCompletionHandle.into(), KeyContext.into(), ApcContext.into(), IoStatusBlock.into(), Timeout.into()]));
        out
    }

    pub fn ZwRemoveIoCompletionEx(&mut self, IoCompletionHandle: impl Into<IRegOrImm<u64>>, IoCompletionInformation: impl Into<IRegOrImm<u64>>, Count: impl Into<IRegOrImm<u64>>, NumEntriesRemoved: impl Into<IRegOrImm<u64>>, Timeout: impl Into<IRegOrImm<u64>>, Alertable: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwRemoveIoCompletionEx", vec![IoCompletionHandle.into(), IoCompletionInformation.into(), Count.into(), NumEntriesRemoved.into(), Timeout.into(), Alertable.into()]));
        out
    }

    pub fn ZwRemoveProcessDebug(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, DebugObjectHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwRemoveProcessDebug", vec![ProcessHandle.into(), DebugObjectHandle.into()]));
        out
    }

    pub fn ZwRenameKey(&mut self, KeyHandle: impl Into<IRegOrImm<u64>>, NewName: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwRenameKey", vec![KeyHandle.into(), NewName.into()]));
        out
    }

    pub fn ZwRenameTransactionManager(&mut self, LogFileName: impl Into<IRegOrImm<u64>>, ExistingTransactionManagerGuid: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwRenameTransactionManager", vec![LogFileName.into(), ExistingTransactionManagerGuid.into()]));
        out
    }

    pub fn ZwReplaceKey(&mut self, NewFile: impl Into<IRegOrImm<u64>>, TargetHandle: impl Into<IRegOrImm<u64>>, OldFile: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwReplaceKey", vec![NewFile.into(), TargetHandle.into(), OldFile.into()]));
        out
    }

    pub fn ZwReplacePartitionUnit(&mut self, TargetInstancePath: impl Into<IRegOrImm<u64>>, SpareInstancePath: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwReplacePartitionUnit", vec![TargetInstancePath.into(), SpareInstancePath.into(), Flags.into()]));
        out
    }

    pub fn ZwReplyPort(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, ReplyMessage: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwReplyPort", vec![PortHandle.into(), ReplyMessage.into()]));
        out
    }

    pub fn ZwReplyWaitReceivePort(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, PortContext: impl Into<IRegOrImm<u64>>, ReplyMessage: impl Into<IRegOrImm<u64>>, ReceiveMessage: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwReplyWaitReceivePort", vec![PortHandle.into(), PortContext.into(), ReplyMessage.into(), ReceiveMessage.into()]));
        out
    }

    pub fn ZwReplyWaitReceivePortEx(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, PortContext: impl Into<IRegOrImm<u64>>, ReplyMessage: impl Into<IRegOrImm<u64>>, ReceiveMessage: impl Into<IRegOrImm<u64>>, Timeout: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwReplyWaitReceivePortEx", vec![PortHandle.into(), PortContext.into(), ReplyMessage.into(), ReceiveMessage.into(), Timeout.into()]));
        out
    }

    pub fn ZwReplyWaitReplyPort(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, ReplyMessage: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwReplyWaitReplyPort", vec![PortHandle.into(), ReplyMessage.into()]));
        out
    }

    pub fn ZwRequestPort(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, RequestMessage: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwRequestPort", vec![PortHandle.into(), RequestMessage.into()]));
        out
    }

    pub fn ZwRequestWaitReplyPort(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, RequestMessage: impl Into<IRegOrImm<u64>>, ReplyMessage: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwRequestWaitReplyPort", vec![PortHandle.into(), RequestMessage.into(), ReplyMessage.into()]));
        out
    }

    pub fn ZwRequestWakeupLatency(&mut self, latency: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwRequestWakeupLatency", vec![latency.into()]));
        out
    }

    pub fn ZwResetEvent(&mut self, EventHandle: impl Into<IRegOrImm<u64>>, PreviousState: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwResetEvent", vec![EventHandle.into(), PreviousState.into()]));
        out
    }

    pub fn ZwResetWriteWatch(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, BaseAddress: impl Into<IRegOrImm<u64>>, RegionSize: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwResetWriteWatch", vec![ProcessHandle.into(), BaseAddress.into(), RegionSize.into()]));
        out
    }

    pub fn ZwRestoreKey(&mut self, KeyHandle: impl Into<IRegOrImm<u64>>, FileHandle: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwRestoreKey", vec![KeyHandle.into(), FileHandle.into(), Flags.into()]));
        out
    }

    pub fn ZwResumeProcess(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwResumeProcess", vec![ProcessHandle.into()]));
        out
    }

    pub fn ZwResumeThread(&mut self, ThreadHandle: impl Into<IRegOrImm<u64>>, PreviousSuspendCount: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwResumeThread", vec![ThreadHandle.into(), PreviousSuspendCount.into()]));
        out
    }

    pub fn ZwRevertContainerImpersonation(&mut self, ) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwRevertContainerImpersonation", vec![]));
        out
    }

    pub fn ZwRollbackComplete(&mut self, EnlistmentHandle: impl Into<IRegOrImm<u64>>, TmVirtualClock: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwRollbackComplete", vec![EnlistmentHandle.into(), TmVirtualClock.into()]));
        out
    }

    pub fn ZwRollbackEnlistment(&mut self, EnlistmentHandle: impl Into<IRegOrImm<u64>>, TmVirtualClock: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwRollbackEnlistment", vec![EnlistmentHandle.into(), TmVirtualClock.into()]));
        out
    }

    pub fn ZwRollbackTransaction(&mut self, TransactionHandle: impl Into<IRegOrImm<u64>>, Wait: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwRollbackTransaction", vec![TransactionHandle.into(), Wait.into()]));
        out
    }

    pub fn ZwRollforwardTransactionManager(&mut self, TransactionManagerHandle: impl Into<IRegOrImm<u64>>, TmVirtualClock: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwRollforwardTransactionManager", vec![TransactionManagerHandle.into(), TmVirtualClock.into()]));
        out
    }

    pub fn ZwSaveKey(&mut self, KeyHandle: impl Into<IRegOrImm<u64>>, FileHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSaveKey", vec![KeyHandle.into(), FileHandle.into()]));
        out
    }

    pub fn ZwSaveKeyEx(&mut self, KeyHandle: impl Into<IRegOrImm<u64>>, FileHandle: impl Into<IRegOrImm<u64>>, Format: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSaveKeyEx", vec![KeyHandle.into(), FileHandle.into(), Format.into()]));
        out
    }

    pub fn ZwSaveMergedKeys(&mut self, HighPrecedenceKeyHandle: impl Into<IRegOrImm<u64>>, LowPrecedenceKeyHandle: impl Into<IRegOrImm<u64>>, FileHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSaveMergedKeys", vec![HighPrecedenceKeyHandle.into(), LowPrecedenceKeyHandle.into(), FileHandle.into()]));
        out
    }

    pub fn ZwSecureConnectPort(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, PortName: impl Into<IRegOrImm<u64>>, SecurityQos: impl Into<IRegOrImm<u64>>, ClientView: impl Into<IRegOrImm<u64>>, RequiredServerSid: impl Into<IRegOrImm<u64>>, ServerView: impl Into<IRegOrImm<u64>>, MaxMessageLength: impl Into<IRegOrImm<u64>>, ConnectionInformation: impl Into<IRegOrImm<u64>>, ConnectionInformationLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSecureConnectPort", vec![PortHandle.into(), PortName.into(), SecurityQos.into(), ClientView.into(), RequiredServerSid.into(), ServerView.into(), MaxMessageLength.into(), ConnectionInformation.into(), ConnectionInformationLength.into()]));
        out
    }

    pub fn ZwSerializeBoot(&mut self, ) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSerializeBoot", vec![]));
        out
    }

    pub fn ZwSetBootEntryOrder(&mut self, Ids: impl Into<IRegOrImm<u64>>, Count: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetBootEntryOrder", vec![Ids.into(), Count.into()]));
        out
    }

    pub fn ZwSetBootOptions(&mut self, BootOptions: impl Into<IRegOrImm<u64>>, FieldsToChange: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetBootOptions", vec![BootOptions.into(), FieldsToChange.into()]));
        out
    }

    pub fn ZwSetCachedSigningLevel(&mut self, Flags: impl Into<IRegOrImm<u64>>, InputSigningLevel: impl Into<IRegOrImm<u64>>, SourceFiles: impl Into<IRegOrImm<u64>>, SourceFileCount: impl Into<IRegOrImm<u64>>, TargetFile: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetCachedSigningLevel", vec![Flags.into(), InputSigningLevel.into(), SourceFiles.into(), SourceFileCount.into(), TargetFile.into()]));
        out
    }

    pub fn ZwSetContextThread(&mut self, ThreadHandle: impl Into<IRegOrImm<u64>>, ThreadContext: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetContextThread", vec![ThreadHandle.into(), ThreadContext.into()]));
        out
    }

    pub fn ZwSetDebugFilterState(&mut self, ComponentId: impl Into<IRegOrImm<u64>>, Level: impl Into<IRegOrImm<u64>>, State: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetDebugFilterState", vec![ComponentId.into(), Level.into(), State.into()]));
        out
    }

    pub fn ZwSetDefaultHardErrorPort(&mut self, DefaultHardErrorPort: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetDefaultHardErrorPort", vec![DefaultHardErrorPort.into()]));
        out
    }

    pub fn ZwSetDefaultLocale(&mut self, UserProfile: impl Into<IRegOrImm<u64>>, DefaultLocaleId: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetDefaultLocale", vec![UserProfile.into(), DefaultLocaleId.into()]));
        out
    }

    pub fn ZwSetDefaultUILanguage(&mut self, DefaultUILanguageId: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetDefaultUILanguage", vec![DefaultUILanguageId.into()]));
        out
    }

    pub fn ZwSetDriverEntryOrder(&mut self, Ids: impl Into<IRegOrImm<u64>>, Count: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetDriverEntryOrder", vec![Ids.into(), Count.into()]));
        out
    }

    pub fn ZwSetEaFile(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetEaFile", vec![FileHandle.into(), IoStatusBlock.into(), Buffer.into(), Length.into()]));
        out
    }

    pub fn ZwSetEvent(&mut self, EventHandle: impl Into<IRegOrImm<u64>>, PreviousState: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetEvent", vec![EventHandle.into(), PreviousState.into()]));
        out
    }

    pub fn ZwSetEventBoostPriority(&mut self, EventHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetEventBoostPriority", vec![EventHandle.into()]));
        out
    }

    pub fn ZwSetHighEventPair(&mut self, EventPairHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetHighEventPair", vec![EventPairHandle.into()]));
        out
    }

    pub fn ZwSetHighWaitLowEventPair(&mut self, EventPairHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetHighWaitLowEventPair", vec![EventPairHandle.into()]));
        out
    }

    pub fn ZwSetInformationDebugObject(&mut self, DebugObjectHandle: impl Into<IRegOrImm<u64>>, DebugObjectInformationClass: impl Into<IRegOrImm<u64>>, DebugInformation: impl Into<IRegOrImm<u64>>, DebugInformationLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetInformationDebugObject", vec![DebugObjectHandle.into(), DebugObjectInformationClass.into(), DebugInformation.into(), DebugInformationLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwSetInformationEnlistment(&mut self, EnlistmentHandle: impl Into<IRegOrImm<u64>>, EnlistmentInformationClass: impl Into<IRegOrImm<u64>>, EnlistmentInformation: impl Into<IRegOrImm<u64>>, EnlistmentInformationLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetInformationEnlistment", vec![EnlistmentHandle.into(), EnlistmentInformationClass.into(), EnlistmentInformation.into(), EnlistmentInformationLength.into()]));
        out
    }

    pub fn ZwSetInformationFile(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, FileInformation: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, FileInformationClass: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetInformationFile", vec![FileHandle.into(), IoStatusBlock.into(), FileInformation.into(), Length.into(), FileInformationClass.into()]));
        out
    }

    pub fn ZwSetInformationIoRing(&mut self, IoRingHandle: impl Into<IRegOrImm<u64>>, IoRingInformationClass: impl Into<IRegOrImm<u64>>, IoRingInformationLength: impl Into<IRegOrImm<u64>>, IoRingInformation: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetInformationIoRing", vec![IoRingHandle.into(), IoRingInformationClass.into(), IoRingInformationLength.into(), IoRingInformation.into()]));
        out
    }

    pub fn ZwSetInformationJobObject(&mut self, JobHandle: impl Into<IRegOrImm<u64>>, JobObjectInformationClass: impl Into<IRegOrImm<u64>>, JobObjectInformation: impl Into<IRegOrImm<u64>>, JobObjectInformationLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetInformationJobObject", vec![JobHandle.into(), JobObjectInformationClass.into(), JobObjectInformation.into(), JobObjectInformationLength.into()]));
        out
    }

    pub fn ZwSetInformationKey(&mut self, KeyHandle: impl Into<IRegOrImm<u64>>, KeySetInformationClass: impl Into<IRegOrImm<u64>>, KeySetInformation: impl Into<IRegOrImm<u64>>, KeySetInformationLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetInformationKey", vec![KeyHandle.into(), KeySetInformationClass.into(), KeySetInformation.into(), KeySetInformationLength.into()]));
        out
    }

    pub fn ZwSetInformationObject(&mut self, Handle: impl Into<IRegOrImm<u64>>, ObjectInformationClass: impl Into<IRegOrImm<u64>>, ObjectInformation: impl Into<IRegOrImm<u64>>, ObjectInformationLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetInformationObject", vec![Handle.into(), ObjectInformationClass.into(), ObjectInformation.into(), ObjectInformationLength.into()]));
        out
    }

    pub fn ZwSetInformationProcess(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, ProcessInformationClass: impl Into<IRegOrImm<u64>>, ProcessInformation: impl Into<IRegOrImm<u64>>, ProcessInformationLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetInformationProcess", vec![ProcessHandle.into(), ProcessInformationClass.into(), ProcessInformation.into(), ProcessInformationLength.into()]));
        out
    }

    pub fn ZwSetInformationResourceManager(&mut self, ResourceManagerHandle: impl Into<IRegOrImm<u64>>, ResourceManagerInformationClass: impl Into<IRegOrImm<u64>>, ResourceManagerInformation: impl Into<IRegOrImm<u64>>, ResourceManagerInformationLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetInformationResourceManager", vec![ResourceManagerHandle.into(), ResourceManagerInformationClass.into(), ResourceManagerInformation.into(), ResourceManagerInformationLength.into()]));
        out
    }

    pub fn ZwSetInformationSymbolicLink(&mut self, LinkHandle: impl Into<IRegOrImm<u64>>, SymbolicLinkInformationClass: impl Into<IRegOrImm<u64>>, SymbolicLinkInformation: impl Into<IRegOrImm<u64>>, SymbolicLinkInformationLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetInformationSymbolicLink", vec![LinkHandle.into(), SymbolicLinkInformationClass.into(), SymbolicLinkInformation.into(), SymbolicLinkInformationLength.into()]));
        out
    }

    pub fn ZwSetInformationThread(&mut self, ThreadHandle: impl Into<IRegOrImm<u64>>, ThreadInformationClass: impl Into<IRegOrImm<u64>>, ThreadInformation: impl Into<IRegOrImm<u64>>, ThreadInformationLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetInformationThread", vec![ThreadHandle.into(), ThreadInformationClass.into(), ThreadInformation.into(), ThreadInformationLength.into()]));
        out
    }

    pub fn ZwSetInformationToken(&mut self, TokenHandle: impl Into<IRegOrImm<u64>>, TokenInformationClass: impl Into<IRegOrImm<u64>>, TokenInformation: impl Into<IRegOrImm<u64>>, TokenInformationLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetInformationToken", vec![TokenHandle.into(), TokenInformationClass.into(), TokenInformation.into(), TokenInformationLength.into()]));
        out
    }

    pub fn ZwSetInformationTransaction(&mut self, TransactionHandle: impl Into<IRegOrImm<u64>>, TransactionInformationClass: impl Into<IRegOrImm<u64>>, TransactionInformation: impl Into<IRegOrImm<u64>>, TransactionInformationLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetInformationTransaction", vec![TransactionHandle.into(), TransactionInformationClass.into(), TransactionInformation.into(), TransactionInformationLength.into()]));
        out
    }

    pub fn ZwSetInformationTransactionManager(&mut self, TmHandle: impl Into<IRegOrImm<u64>>, TransactionManagerInformationClass: impl Into<IRegOrImm<u64>>, TransactionManagerInformation: impl Into<IRegOrImm<u64>>, TransactionManagerInformationLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetInformationTransactionManager", vec![TmHandle.into(), TransactionManagerInformationClass.into(), TransactionManagerInformation.into(), TransactionManagerInformationLength.into()]));
        out
    }

    pub fn ZwSetInformationVirtualMemory(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, VmInformationClass: impl Into<IRegOrImm<u64>>, NumberOfEntries: impl Into<IRegOrImm<u64>>, VirtualAddresses: impl Into<IRegOrImm<u64>>, VmInformation: impl Into<IRegOrImm<u64>>, VmInformationLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetInformationVirtualMemory", vec![ProcessHandle.into(), VmInformationClass.into(), NumberOfEntries.into(), VirtualAddresses.into(), VmInformation.into(), VmInformationLength.into()]));
        out
    }

    pub fn ZwSetInformationWorkerFactory(&mut self, WorkerFactoryHandle: impl Into<IRegOrImm<u64>>, WorkerFactoryInformationClass: impl Into<IRegOrImm<u64>>, WorkerFactoryInformation: impl Into<IRegOrImm<u64>>, WorkerFactoryInformationLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetInformationWorkerFactory", vec![WorkerFactoryHandle.into(), WorkerFactoryInformationClass.into(), WorkerFactoryInformation.into(), WorkerFactoryInformationLength.into()]));
        out
    }

    pub fn ZwSetIntervalProfile(&mut self, Interval: impl Into<IRegOrImm<u64>>, Source: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetIntervalProfile", vec![Interval.into(), Source.into()]));
        out
    }

    pub fn ZwSetIoCompletion(&mut self, IoCompletionHandle: impl Into<IRegOrImm<u64>>, KeyContext: impl Into<IRegOrImm<u64>>, ApcContext: impl Into<IRegOrImm<u64>>, IoStatus: impl Into<IRegOrImm<u64>>, IoStatusInformation: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetIoCompletion", vec![IoCompletionHandle.into(), KeyContext.into(), ApcContext.into(), IoStatus.into(), IoStatusInformation.into()]));
        out
    }

    pub fn ZwSetIoCompletionEx(&mut self, IoCompletionHandle: impl Into<IRegOrImm<u64>>, IoCompletionPacketHandle: impl Into<IRegOrImm<u64>>, KeyContext: impl Into<IRegOrImm<u64>>, ApcContext: impl Into<IRegOrImm<u64>>, IoStatus: impl Into<IRegOrImm<u64>>, IoStatusInformation: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetIoCompletionEx", vec![IoCompletionHandle.into(), IoCompletionPacketHandle.into(), KeyContext.into(), ApcContext.into(), IoStatus.into(), IoStatusInformation.into()]));
        out
    }

    pub fn ZwSetIRTimer(&mut self, TimerHandle: impl Into<IRegOrImm<u64>>, DueTime: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetIRTimer", vec![TimerHandle.into(), DueTime.into()]));
        out
    }

    pub fn ZwSetLdtEntries(&mut self, Selector0: impl Into<IRegOrImm<u64>>, Entry0Low: impl Into<IRegOrImm<u64>>, Entry0Hi: impl Into<IRegOrImm<u64>>, Selector1: impl Into<IRegOrImm<u64>>, Entry1Low: impl Into<IRegOrImm<u64>>, Entry1Hi: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetLdtEntries", vec![Selector0.into(), Entry0Low.into(), Entry0Hi.into(), Selector1.into(), Entry1Low.into(), Entry1Hi.into()]));
        out
    }

    pub fn ZwSetLowEventPair(&mut self, EventPairHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetLowEventPair", vec![EventPairHandle.into()]));
        out
    }

    pub fn ZwSetLowWaitHighEventPair(&mut self, EventPairHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetLowWaitHighEventPair", vec![EventPairHandle.into()]));
        out
    }

    pub fn ZwSetQuotaInformationFile(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetQuotaInformationFile", vec![FileHandle.into(), IoStatusBlock.into(), Buffer.into(), Length.into()]));
        out
    }

    pub fn ZwSetSecurityObject(&mut self, Handle: impl Into<IRegOrImm<u64>>, SecurityInformation: impl Into<IRegOrImm<u64>>, SecurityDescriptor: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetSecurityObject", vec![Handle.into(), SecurityInformation.into(), SecurityDescriptor.into()]));
        out
    }

    pub fn ZwSetSystemEnvironmentValue(&mut self, VariableName: impl Into<IRegOrImm<u64>>, VariableValue: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetSystemEnvironmentValue", vec![VariableName.into(), VariableValue.into()]));
        out
    }

    pub fn ZwSetSystemEnvironmentValueEx(&mut self, VariableName: impl Into<IRegOrImm<u64>>, VendorGuid: impl Into<IRegOrImm<u64>>, Value: impl Into<IRegOrImm<u64>>, ValueLength: impl Into<IRegOrImm<u64>>, Attributes: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetSystemEnvironmentValueEx", vec![VariableName.into(), VendorGuid.into(), Value.into(), ValueLength.into(), Attributes.into()]));
        out
    }

    pub fn ZwSetSystemInformation(&mut self, SystemInformationClass: impl Into<IRegOrImm<u64>>, SystemInformation: impl Into<IRegOrImm<u64>>, SystemInformationLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetSystemInformation", vec![SystemInformationClass.into(), SystemInformation.into(), SystemInformationLength.into()]));
        out
    }

    pub fn ZwSetSystemPowerState(&mut self, SystemAction: impl Into<IRegOrImm<u64>>, LightestSystemState: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetSystemPowerState", vec![SystemAction.into(), LightestSystemState.into(), Flags.into()]));
        out
    }

    pub fn ZwSetSystemTime(&mut self, SystemTime: impl Into<IRegOrImm<u64>>, PreviousTime: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetSystemTime", vec![SystemTime.into(), PreviousTime.into()]));
        out
    }

    pub fn ZwSetThreadExecutionState(&mut self, NewFlags: impl Into<IRegOrImm<u64>>, PreviousFlags: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetThreadExecutionState", vec![NewFlags.into(), PreviousFlags.into()]));
        out
    }

    pub fn ZwSetTimer(&mut self, TimerHandle: impl Into<IRegOrImm<u64>>, DueTime: impl Into<IRegOrImm<u64>>, TimerApcRoutine: impl Into<IRegOrImm<u64>>, TimerContext: impl Into<IRegOrImm<u64>>, ResumeTimer: impl Into<IRegOrImm<u64>>, Period: impl Into<IRegOrImm<u64>>, PreviousState: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetTimer", vec![TimerHandle.into(), DueTime.into(), TimerApcRoutine.into(), TimerContext.into(), ResumeTimer.into(), Period.into(), PreviousState.into()]));
        out
    }

    pub fn ZwSetTimer2(&mut self, TimerHandle: impl Into<IRegOrImm<u64>>, DueTime: impl Into<IRegOrImm<u64>>, Period: impl Into<IRegOrImm<u64>>, Parameters: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetTimer2", vec![TimerHandle.into(), DueTime.into(), Period.into(), Parameters.into()]));
        out
    }

    pub fn ZwSetTimerEx(&mut self, TimerHandle: impl Into<IRegOrImm<u64>>, TimerSetInformationClass: impl Into<IRegOrImm<u64>>, TimerSetInformation: impl Into<IRegOrImm<u64>>, TimerSetInformationLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetTimerEx", vec![TimerHandle.into(), TimerSetInformationClass.into(), TimerSetInformation.into(), TimerSetInformationLength.into()]));
        out
    }

    pub fn ZwSetTimerResolution(&mut self, DesiredTime: impl Into<IRegOrImm<u64>>, SetResolution: impl Into<IRegOrImm<u64>>, ActualTime: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetTimerResolution", vec![DesiredTime.into(), SetResolution.into(), ActualTime.into()]));
        out
    }

    pub fn ZwSetUuidSeed(&mut self, Seed: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetUuidSeed", vec![Seed.into()]));
        out
    }

    pub fn ZwSetValueKey(&mut self, KeyHandle: impl Into<IRegOrImm<u64>>, ValueName: impl Into<IRegOrImm<u64>>, TitleIndex: impl Into<IRegOrImm<u64>>, Type: impl Into<IRegOrImm<u64>>, Data: impl Into<IRegOrImm<u64>>, DataSize: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetValueKey", vec![KeyHandle.into(), ValueName.into(), TitleIndex.into(), Type.into(), Data.into(), DataSize.into()]));
        out
    }

    pub fn ZwSetVolumeInformationFile(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, FsInformation: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, FsInformationClass: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetVolumeInformationFile", vec![FileHandle.into(), IoStatusBlock.into(), FsInformation.into(), Length.into(), FsInformationClass.into()]));
        out
    }

    pub fn ZwSetWnfProcessNotificationEvent(&mut self, NotificationEvent: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSetWnfProcessNotificationEvent", vec![NotificationEvent.into()]));
        out
    }

    pub fn ZwShutdownSystem(&mut self, Action: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwShutdownSystem", vec![Action.into()]));
        out
    }

    pub fn ZwShutdownWorkerFactory(&mut self, WorkerFactoryHandle: impl Into<IRegOrImm<u64>>, PendingWorkerCount: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwShutdownWorkerFactory", vec![WorkerFactoryHandle.into(), PendingWorkerCount.into()]));
        out
    }

    pub fn ZwSignalAndWaitForSingleObject(&mut self, SignalHandle: impl Into<IRegOrImm<u64>>, WaitHandle: impl Into<IRegOrImm<u64>>, Alertable: impl Into<IRegOrImm<u64>>, Timeout: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSignalAndWaitForSingleObject", vec![SignalHandle.into(), WaitHandle.into(), Alertable.into(), Timeout.into()]));
        out
    }

    pub fn ZwSinglePhaseReject(&mut self, EnlistmentHandle: impl Into<IRegOrImm<u64>>, TmVirtualClock: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSinglePhaseReject", vec![EnlistmentHandle.into(), TmVirtualClock.into()]));
        out
    }

    pub fn ZwStartProfile(&mut self, ProfileHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwStartProfile", vec![ProfileHandle.into()]));
        out
    }

    pub fn ZwStopProfile(&mut self, ProfileHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwStopProfile", vec![ProfileHandle.into()]));
        out
    }

    pub fn ZwSubmitIoRing(&mut self, IoRingHandle: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, WaitOperations: impl Into<IRegOrImm<u64>>, Timeout: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSubmitIoRing", vec![IoRingHandle.into(), Flags.into(), WaitOperations.into(), Timeout.into()]));
        out
    }

    pub fn ZwSubscribeWnfStateChange(&mut self, StateName: impl Into<IRegOrImm<u64>>, ChangeStamp: impl Into<IRegOrImm<u64>>, EventMask: impl Into<IRegOrImm<u64>>, SubscriptionId: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSubscribeWnfStateChange", vec![StateName.into(), ChangeStamp.into(), EventMask.into(), SubscriptionId.into()]));
        out
    }

    pub fn ZwSuspendProcess(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSuspendProcess", vec![ProcessHandle.into()]));
        out
    }

    pub fn ZwSuspendThread(&mut self, ThreadHandle: impl Into<IRegOrImm<u64>>, PreviousSuspendCount: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSuspendThread", vec![ThreadHandle.into(), PreviousSuspendCount.into()]));
        out
    }

    pub fn ZwSystemDebugControl(&mut self, Command: impl Into<IRegOrImm<u64>>, InputBuffer: impl Into<IRegOrImm<u64>>, InputBufferLength: impl Into<IRegOrImm<u64>>, OutputBuffer: impl Into<IRegOrImm<u64>>, OutputBufferLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwSystemDebugControl", vec![Command.into(), InputBuffer.into(), InputBufferLength.into(), OutputBuffer.into(), OutputBufferLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwTerminateEnclave(&mut self, BaseAddress: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwTerminateEnclave", vec![BaseAddress.into(), Flags.into()]));
        out
    }

    pub fn ZwTerminateJobObject(&mut self, JobHandle: impl Into<IRegOrImm<u64>>, ExitStatus: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwTerminateJobObject", vec![JobHandle.into(), ExitStatus.into()]));
        out
    }

    pub fn ZwTerminateProcess(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, ExitStatus: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwTerminateProcess", vec![ProcessHandle.into(), ExitStatus.into()]));
        out
    }

    pub fn ZwTerminateThread(&mut self, ThreadHandle: impl Into<IRegOrImm<u64>>, ExitStatus: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwTerminateThread", vec![ThreadHandle.into(), ExitStatus.into()]));
        out
    }

    pub fn ZwTestAlert(&mut self, ) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwTestAlert", vec![]));
        out
    }

    pub fn ZwThawRegistry(&mut self, ) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwThawRegistry", vec![]));
        out
    }

    pub fn ZwThawTransactions(&mut self, ) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwThawTransactions", vec![]));
        out
    }

    pub fn ZwTraceControl(&mut self, TraceControlCode: impl Into<IRegOrImm<u64>>, InputBuffer: impl Into<IRegOrImm<u64>>, InputBufferLength: impl Into<IRegOrImm<u64>>, OutputBuffer: impl Into<IRegOrImm<u64>>, OutputBufferLength: impl Into<IRegOrImm<u64>>, ReturnLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwTraceControl", vec![TraceControlCode.into(), InputBuffer.into(), InputBufferLength.into(), OutputBuffer.into(), OutputBufferLength.into(), ReturnLength.into()]));
        out
    }

    pub fn ZwTraceEvent(&mut self, TraceHandle: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>, FieldSize: impl Into<IRegOrImm<u64>>, Fields: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwTraceEvent", vec![TraceHandle.into(), Flags.into(), FieldSize.into(), Fields.into()]));
        out
    }

    pub fn ZwTranslateFilePath(&mut self, InputFilePath: impl Into<IRegOrImm<u64>>, OutputType: impl Into<IRegOrImm<u64>>, OutputFilePath: impl Into<IRegOrImm<u64>>, OutputFilePathLength: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwTranslateFilePath", vec![InputFilePath.into(), OutputType.into(), OutputFilePath.into(), OutputFilePathLength.into()]));
        out
    }

    pub fn ZwUmsThreadYield(&mut self, SchedulerParam: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwUmsThreadYield", vec![SchedulerParam.into()]));
        out
    }

    pub fn ZwUnloadDriver(&mut self, DriverServiceName: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwUnloadDriver", vec![DriverServiceName.into()]));
        out
    }

    pub fn ZwUnloadKey(&mut self, TargetKey: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwUnloadKey", vec![TargetKey.into()]));
        out
    }

    pub fn ZwUnloadKey2(&mut self, TargetKey: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwUnloadKey2", vec![TargetKey.into(), Flags.into()]));
        out
    }

    pub fn ZwUnloadKeyEx(&mut self, TargetKey: impl Into<IRegOrImm<u64>>, Event: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwUnloadKeyEx", vec![TargetKey.into(), Event.into()]));
        out
    }

    pub fn ZwUnlockFile(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, ByteOffset: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, Key: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwUnlockFile", vec![FileHandle.into(), IoStatusBlock.into(), ByteOffset.into(), Length.into(), Key.into()]));
        out
    }

    pub fn ZwUnlockVirtualMemory(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, BaseAddress: impl Into<IRegOrImm<u64>>, RegionSize: impl Into<IRegOrImm<u64>>, MapType: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwUnlockVirtualMemory", vec![ProcessHandle.into(), BaseAddress.into(), RegionSize.into(), MapType.into()]));
        out
    }

    pub fn ZwUnmapViewOfSection(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, BaseAddress: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwUnmapViewOfSection", vec![ProcessHandle.into(), BaseAddress.into()]));
        out
    }

    pub fn ZwUnmapViewOfSectionEx(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, BaseAddress: impl Into<IRegOrImm<u64>>, Flags: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwUnmapViewOfSectionEx", vec![ProcessHandle.into(), BaseAddress.into(), Flags.into()]));
        out
    }

    pub fn ZwUnsubscribeWnfStateChange(&mut self, StateName: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwUnsubscribeWnfStateChange", vec![StateName.into()]));
        out
    }

    pub fn ZwUpdateWnfStateData(&mut self, StateName: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, TypeId: impl Into<IRegOrImm<u64>>, ExplicitScope: impl Into<IRegOrImm<u64>>, MatchingChangeStamp: impl Into<IRegOrImm<u64>>, CheckStamp: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwUpdateWnfStateData", vec![StateName.into(), Buffer.into(), Length.into(), TypeId.into(), ExplicitScope.into(), MatchingChangeStamp.into(), CheckStamp.into()]));
        out
    }

    pub fn ZwVdmControl(&mut self, Service: impl Into<IRegOrImm<u64>>, ServiceData: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwVdmControl", vec![Service.into(), ServiceData.into()]));
        out
    }

    pub fn ZwWaitForAlertByThreadId(&mut self, Address: impl Into<IRegOrImm<u64>>, Timeout: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwWaitForAlertByThreadId", vec![Address.into(), Timeout.into()]));
        out
    }

    pub fn ZwWaitForDebugEvent(&mut self, DebugObjectHandle: impl Into<IRegOrImm<u64>>, Alertable: impl Into<IRegOrImm<u64>>, Timeout: impl Into<IRegOrImm<u64>>, WaitStateChange: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwWaitForDebugEvent", vec![DebugObjectHandle.into(), Alertable.into(), Timeout.into(), WaitStateChange.into()]));
        out
    }

    pub fn ZwWaitForKeyedEvent(&mut self, KeyedEventHandle: impl Into<IRegOrImm<u64>>, KeyValue: impl Into<IRegOrImm<u64>>, Alertable: impl Into<IRegOrImm<u64>>, Timeout: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwWaitForKeyedEvent", vec![KeyedEventHandle.into(), KeyValue.into(), Alertable.into(), Timeout.into()]));
        out
    }

    pub fn ZwWaitForMultipleObjects(&mut self, Count: impl Into<IRegOrImm<u64>>, Handles: impl Into<IRegOrImm<u64>>, WaitType: impl Into<IRegOrImm<u64>>, Alertable: impl Into<IRegOrImm<u64>>, Timeout: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwWaitForMultipleObjects", vec![Count.into(), Handles.into(), WaitType.into(), Alertable.into(), Timeout.into()]));
        out
    }

    pub fn ZwWaitForMultipleObjects32(&mut self, Count: impl Into<IRegOrImm<u64>>, Handles: impl Into<IRegOrImm<u64>>, WaitType: impl Into<IRegOrImm<u64>>, Alertable: impl Into<IRegOrImm<u64>>, Timeout: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwWaitForMultipleObjects32", vec![Count.into(), Handles.into(), WaitType.into(), Alertable.into(), Timeout.into()]));
        out
    }

    pub fn ZwWaitForSingleObject(&mut self, Handle: impl Into<IRegOrImm<u64>>, Alertable: impl Into<IRegOrImm<u64>>, Timeout: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwWaitForSingleObject", vec![Handle.into(), Alertable.into(), Timeout.into()]));
        out
    }

    pub fn ZwWaitForWorkViaWorkerFactory(&mut self, WorkerFactoryHandle: impl Into<IRegOrImm<u64>>, MiniPackets: impl Into<IRegOrImm<u64>>, Count: impl Into<IRegOrImm<u64>>, PacketsReturned: impl Into<IRegOrImm<u64>>, DeferredWork: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwWaitForWorkViaWorkerFactory", vec![WorkerFactoryHandle.into(), MiniPackets.into(), Count.into(), PacketsReturned.into(), DeferredWork.into()]));
        out
    }

    pub fn ZwWaitHighEventPair(&mut self, EventPairHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwWaitHighEventPair", vec![EventPairHandle.into()]));
        out
    }

    pub fn ZwWaitLowEventPair(&mut self, EventPairHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwWaitLowEventPair", vec![EventPairHandle.into()]));
        out
    }

    pub fn ZwWorkerFactoryWorkerReady(&mut self, WorkerFactoryHandle: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwWorkerFactoryWorkerReady", vec![WorkerFactoryHandle.into()]));
        out
    }

    pub fn ZwWriteFile(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, Event: impl Into<IRegOrImm<u64>>, ApcRoutine: impl Into<IRegOrImm<u64>>, ApcContext: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, ByteOffset: impl Into<IRegOrImm<u64>>, Key: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwWriteFile", vec![FileHandle.into(), Event.into(), ApcRoutine.into(), ApcContext.into(), IoStatusBlock.into(), Buffer.into(), Length.into(), ByteOffset.into(), Key.into()]));
        out
    }

    pub fn ZwWriteFileGather(&mut self, FileHandle: impl Into<IRegOrImm<u64>>, Event: impl Into<IRegOrImm<u64>>, ApcRoutine: impl Into<IRegOrImm<u64>>, ApcContext: impl Into<IRegOrImm<u64>>, IoStatusBlock: impl Into<IRegOrImm<u64>>, SegmentArray: impl Into<IRegOrImm<u64>>, Length: impl Into<IRegOrImm<u64>>, ByteOffset: impl Into<IRegOrImm<u64>>, Key: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwWriteFileGather", vec![FileHandle.into(), Event.into(), ApcRoutine.into(), ApcContext.into(), IoStatusBlock.into(), SegmentArray.into(), Length.into(), ByteOffset.into(), Key.into()]));
        out
    }

    pub fn ZwWriteRequestData(&mut self, PortHandle: impl Into<IRegOrImm<u64>>, Message: impl Into<IRegOrImm<u64>>, DataEntryIndex: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>, BufferSize: impl Into<IRegOrImm<u64>>, NumberOfBytesWritten: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwWriteRequestData", vec![PortHandle.into(), Message.into(), DataEntryIndex.into(), Buffer.into(), BufferSize.into(), NumberOfBytesWritten.into()]));
        out
    }

    pub fn ZwWriteVirtualMemory(&mut self, ProcessHandle: impl Into<IRegOrImm<u64>>, BaseAddress: impl Into<IRegOrImm<u64>>, Buffer: impl Into<IRegOrImm<u64>>, BufferSize: impl Into<IRegOrImm<u64>>, NumberOfBytesWritten: impl Into<IRegOrImm<u64>>) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwWriteVirtualMemory", vec![ProcessHandle.into(), BaseAddress.into(), Buffer.into(), BufferSize.into(), NumberOfBytesWritten.into()]));
        out
    }

    pub fn ZwYieldExecution(&mut self, ) -> IReg {
        let out = self.reg();
        self.add_insn(Insn::Syscall(out, "ZwYieldExecution", vec![]));
        out
    }

}
