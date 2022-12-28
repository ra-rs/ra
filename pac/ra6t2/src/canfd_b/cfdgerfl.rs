#[doc = "Register `CFDGERFL` reader"]
pub struct R(crate::R<CFDGERFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDGERFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDGERFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDGERFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDGERFL` writer"]
pub struct W(crate::W<CFDGERFL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDGERFL_SPEC>;
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
impl From<crate::W<CFDGERFL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDGERFL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEF` reader - DLC Error Flag"]
pub type DEF_R = crate::BitReader<DEF_A>;
#[doc = "DLC Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEF_A {
    #[doc = "0: DLC error not detected"]
    _0 = 0,
    #[doc = "1: DLC error detected"]
    _1 = 1,
}
impl From<DEF_A> for bool {
    #[inline(always)]
    fn from(variant: DEF_A) -> Self {
        variant as u8 != 0
    }
}
impl DEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEF_A {
        match self.bits {
            false => DEF_A::_0,
            true => DEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DEF_A::_1
    }
}
#[doc = "Field `DEF` writer - DLC Error Flag"]
pub type DEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGERFL_SPEC, DEF_A, O>;
impl<'a, const O: u8> DEF_W<'a, O> {
    #[doc = "DLC error not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DEF_A::_0)
    }
    #[doc = "DLC error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DEF_A::_1)
    }
}
#[doc = "Field `MES` reader - Message Lost Error Status"]
pub type MES_R = crate::BitReader<MES_A>;
#[doc = "Message Lost Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MES_A {
    #[doc = "0: Message lost error not detected"]
    _0 = 0,
    #[doc = "1: Message lost error detected"]
    _1 = 1,
}
impl From<MES_A> for bool {
    #[inline(always)]
    fn from(variant: MES_A) -> Self {
        variant as u8 != 0
    }
}
impl MES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MES_A {
        match self.bits {
            false => MES_A::_0,
            true => MES_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MES_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MES_A::_1
    }
}
#[doc = "Field `THLES` reader - TX History List Entry Lost Error Status"]
pub type THLES_R = crate::BitReader<THLES_A>;
#[doc = "TX History List Entry Lost Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THLES_A {
    #[doc = "0: TX history list entry lost error not detected"]
    _0 = 0,
    #[doc = "1: TX history list entry lost error detected"]
    _1 = 1,
}
impl From<THLES_A> for bool {
    #[inline(always)]
    fn from(variant: THLES_A) -> Self {
        variant as u8 != 0
    }
}
impl THLES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THLES_A {
        match self.bits {
            false => THLES_A::_0,
            true => THLES_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == THLES_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == THLES_A::_1
    }
}
#[doc = "Field `CMPOF` reader - CANFD Message Payload Overflow Flag"]
pub type CMPOF_R = crate::BitReader<CMPOF_A>;
#[doc = "CANFD Message Payload Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPOF_A {
    #[doc = "0: CANFD message payload overflow not detected"]
    _0 = 0,
    #[doc = "1: CANFD message payload overflow detected"]
    _1 = 1,
}
impl From<CMPOF_A> for bool {
    #[inline(always)]
    fn from(variant: CMPOF_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPOF_A {
        match self.bits {
            false => CMPOF_A::_0,
            true => CMPOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPOF_A::_1
    }
}
#[doc = "Field `CMPOF` writer - CANFD Message Payload Overflow Flag"]
pub type CMPOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGERFL_SPEC, CMPOF_A, O>;
impl<'a, const O: u8> CMPOF_W<'a, O> {
    #[doc = "CANFD message payload overflow not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPOF_A::_0)
    }
    #[doc = "CANFD message payload overflow detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPOF_A::_1)
    }
}
#[doc = "Field `EEF0` reader - ECC Error Flag"]
pub type EEF0_R = crate::BitReader<EEF0_A>;
#[doc = "ECC Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEF0_A {
    #[doc = "0: ECC error not detected during TX-SCAN"]
    _0 = 0,
    #[doc = "1: ECC error detected during TX-SCAN"]
    _1 = 1,
}
impl From<EEF0_A> for bool {
    #[inline(always)]
    fn from(variant: EEF0_A) -> Self {
        variant as u8 != 0
    }
}
impl EEF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEF0_A {
        match self.bits {
            false => EEF0_A::_0,
            true => EEF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEF0_A::_1
    }
}
#[doc = "Field `EEF0` writer - ECC Error Flag"]
pub type EEF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGERFL_SPEC, EEF0_A, O>;
impl<'a, const O: u8> EEF0_W<'a, O> {
    #[doc = "ECC error not detected during TX-SCAN"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEF0_A::_0)
    }
    #[doc = "ECC error detected during TX-SCAN"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEF0_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - DLC Error Flag"]
    #[inline(always)]
    pub fn def(&self) -> DEF_R {
        DEF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Message Lost Error Status"]
    #[inline(always)]
    pub fn mes(&self) -> MES_R {
        MES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX History List Entry Lost Error Status"]
    #[inline(always)]
    pub fn thles(&self) -> THLES_R {
        THLES_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CANFD Message Payload Overflow Flag"]
    #[inline(always)]
    pub fn cmpof(&self) -> CMPOF_R {
        CMPOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - ECC Error Flag"]
    #[inline(always)]
    pub fn eef0(&self) -> EEF0_R {
        EEF0_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DLC Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn def(&mut self) -> DEF_W<0> {
        DEF_W::new(self)
    }
    #[doc = "Bit 3 - CANFD Message Payload Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpof(&mut self) -> CMPOF_W<3> {
        CMPOF_W::new(self)
    }
    #[doc = "Bit 16 - ECC Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn eef0(&mut self) -> EEF0_W<16> {
        EEF0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Error Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdgerfl](index.html) module"]
pub struct CFDGERFL_SPEC;
impl crate::RegisterSpec for CFDGERFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdgerfl::R](R) reader structure"]
impl crate::Readable for CFDGERFL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdgerfl::W](W) writer structure"]
impl crate::Writable for CFDGERFL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDGERFL to value 0"]
impl crate::Resettable for CFDGERFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
