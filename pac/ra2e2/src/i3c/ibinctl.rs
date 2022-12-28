#[doc = "Register `IBINCTL` reader"]
pub struct R(crate::R<IBINCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IBINCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IBINCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IBINCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IBINCTL` writer"]
pub struct W(crate::W<IBINCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IBINCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IBINCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IBINCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NRHJCTL` reader - Notify Rejected Hot-Join Control"]
pub type NRHJCTL_R = crate::BitReader<NRHJCTL_A>;
#[doc = "Notify Rejected Hot-Join Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NRHJCTL_A {
    #[doc = "0: Do not pass rejected IBI Status to IBI Queue, if the incoming HotJoin request is NACKed and is autodisabled based on field HJACKCTL of BCTL."]
    _0 = 0,
    #[doc = "1: Pass rejected IBI Status to the IBI Queue, if the incoming Hot Join request is NACKed and is autodisabled based on field HJACKCTL of BCTL."]
    _1 = 1,
}
impl From<NRHJCTL_A> for bool {
    #[inline(always)]
    fn from(variant: NRHJCTL_A) -> Self {
        variant as u8 != 0
    }
}
impl NRHJCTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NRHJCTL_A {
        match self.bits {
            false => NRHJCTL_A::_0,
            true => NRHJCTL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NRHJCTL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NRHJCTL_A::_1
    }
}
#[doc = "Field `NRHJCTL` writer - Notify Rejected Hot-Join Control"]
pub type NRHJCTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IBINCTL_SPEC, NRHJCTL_A, O>;
impl<'a, const O: u8> NRHJCTL_W<'a, O> {
    #[doc = "Do not pass rejected IBI Status to IBI Queue, if the incoming HotJoin request is NACKed and is autodisabled based on field HJACKCTL of BCTL."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NRHJCTL_A::_0)
    }
    #[doc = "Pass rejected IBI Status to the IBI Queue, if the incoming Hot Join request is NACKed and is autodisabled based on field HJACKCTL of BCTL."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NRHJCTL_A::_1)
    }
}
#[doc = "Field `NRMRCTL` reader - Notify Rejected Master Request Control"]
pub type NRMRCTL_R = crate::BitReader<NRMRCTL_A>;
#[doc = "Notify Rejected Master Request Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NRMRCTL_A {
    #[doc = "0: Do not pass rejected IBI Status to IBI Queue/Ring, if the incoming Master Request is NACKed and is auto-disabled based on DVMRRJ field in relevant DAT entry."]
    _0 = 0,
    #[doc = "1: Pass rejected IBI Status to the IBI Queue, if the incoming Master Request is NACKed and is autodisabled based on DVMRRJ field in relevant DAT entry."]
    _1 = 1,
}
impl From<NRMRCTL_A> for bool {
    #[inline(always)]
    fn from(variant: NRMRCTL_A) -> Self {
        variant as u8 != 0
    }
}
impl NRMRCTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NRMRCTL_A {
        match self.bits {
            false => NRMRCTL_A::_0,
            true => NRMRCTL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NRMRCTL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NRMRCTL_A::_1
    }
}
#[doc = "Field `NRMRCTL` writer - Notify Rejected Master Request Control"]
pub type NRMRCTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IBINCTL_SPEC, NRMRCTL_A, O>;
impl<'a, const O: u8> NRMRCTL_W<'a, O> {
    #[doc = "Do not pass rejected IBI Status to IBI Queue/Ring, if the incoming Master Request is NACKed and is auto-disabled based on DVMRRJ field in relevant DAT entry."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NRMRCTL_A::_0)
    }
    #[doc = "Pass rejected IBI Status to the IBI Queue, if the incoming Master Request is NACKed and is autodisabled based on DVMRRJ field in relevant DAT entry."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NRMRCTL_A::_1)
    }
}
#[doc = "Field `NRSIRCTL` reader - Notify Rejected Slave Interrupt Request Control"]
pub type NRSIRCTL_R = crate::BitReader<NRSIRCTL_A>;
#[doc = "Notify Rejected Slave Interrupt Request Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NRSIRCTL_A {
    #[doc = "0: Do not pass rejected IBI Status to the IBI Queue/Rings, if the incoming SIR is NACKed and is auto-disabled based on DVSIRRJ field in relevant DAT entry."]
    _0 = 0,
    #[doc = "1: Pass rejected IBI Status to the IBI Queue/Rings, if the incoming SIR is NACKed and is auto-disabled based on DVSIRRJ field in relevant DAT entry."]
    _1 = 1,
}
impl From<NRSIRCTL_A> for bool {
    #[inline(always)]
    fn from(variant: NRSIRCTL_A) -> Self {
        variant as u8 != 0
    }
}
impl NRSIRCTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NRSIRCTL_A {
        match self.bits {
            false => NRSIRCTL_A::_0,
            true => NRSIRCTL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NRSIRCTL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NRSIRCTL_A::_1
    }
}
#[doc = "Field `NRSIRCTL` writer - Notify Rejected Slave Interrupt Request Control"]
pub type NRSIRCTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IBINCTL_SPEC, NRSIRCTL_A, O>;
impl<'a, const O: u8> NRSIRCTL_W<'a, O> {
    #[doc = "Do not pass rejected IBI Status to the IBI Queue/Rings, if the incoming SIR is NACKed and is auto-disabled based on DVSIRRJ field in relevant DAT entry."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NRSIRCTL_A::_0)
    }
    #[doc = "Pass rejected IBI Status to the IBI Queue/Rings, if the incoming SIR is NACKed and is auto-disabled based on DVSIRRJ field in relevant DAT entry."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NRSIRCTL_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Notify Rejected Hot-Join Control"]
    #[inline(always)]
    pub fn nrhjctl(&self) -> NRHJCTL_R {
        NRHJCTL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Notify Rejected Master Request Control"]
    #[inline(always)]
    pub fn nrmrctl(&self) -> NRMRCTL_R {
        NRMRCTL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Notify Rejected Slave Interrupt Request Control"]
    #[inline(always)]
    pub fn nrsirctl(&self) -> NRSIRCTL_R {
        NRSIRCTL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Notify Rejected Hot-Join Control"]
    #[inline(always)]
    #[must_use]
    pub fn nrhjctl(&mut self) -> NRHJCTL_W<0> {
        NRHJCTL_W::new(self)
    }
    #[doc = "Bit 1 - Notify Rejected Master Request Control"]
    #[inline(always)]
    #[must_use]
    pub fn nrmrctl(&mut self) -> NRMRCTL_W<1> {
        NRMRCTL_W::new(self)
    }
    #[doc = "Bit 3 - Notify Rejected Slave Interrupt Request Control"]
    #[inline(always)]
    #[must_use]
    pub fn nrsirctl(&mut self) -> NRSIRCTL_W<3> {
        NRSIRCTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IBI Notify Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ibinctl](index.html) module"]
pub struct IBINCTL_SPEC;
impl crate::RegisterSpec for IBINCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ibinctl::R](R) reader structure"]
impl crate::Readable for IBINCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ibinctl::W](W) writer structure"]
impl crate::Writable for IBINCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IBINCTL to value 0"]
impl crate::Resettable for IBINCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
