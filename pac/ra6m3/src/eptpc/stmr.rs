#[doc = "Register `STMR` reader"]
pub struct R(crate::R<STMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STMR` writer"]
pub struct W(crate::W<STMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STMR_SPEC>;
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
impl From<crate::W<STMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WINT` reader - Worst 10 Acquisition Time"]
pub type WINT_R = crate::FieldReader<u8, WINT_A>;
#[doc = "Worst 10 Acquisition Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WINT_A {
    #[doc = "0: The worst 10 values are not acquired."]
    _0X00 = 0,
}
impl From<WINT_A> for u8 {
    #[inline(always)]
    fn from(variant: WINT_A) -> Self {
        variant as _
    }
}
impl WINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WINT_A> {
        match self.bits {
            0 => Some(WINT_A::_0X00),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == WINT_A::_0X00
    }
}
#[doc = "Field `WINT` writer - Worst 10 Acquisition Time"]
pub type WINT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STMR_SPEC, u8, WINT_A, 8, O>;
impl<'a, const O: u8> WINT_W<'a, O> {
    #[doc = "The worst 10 values are not acquired."]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(WINT_A::_0X00)
    }
}
#[doc = "Field `CMOD` reader - Time Synchronization Correction Mode"]
pub type CMOD_R = crate::BitReader<CMOD_A>;
#[doc = "Time Synchronization Correction Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMOD_A {
    #[doc = "0: Mode 1"]
    _0 = 0,
    #[doc = "1: Mode 2"]
    _1 = 1,
}
impl From<CMOD_A> for bool {
    #[inline(always)]
    fn from(variant: CMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl CMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMOD_A {
        match self.bits {
            false => CMOD_A::_0,
            true => CMOD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMOD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMOD_A::_1
    }
}
#[doc = "Field `CMOD` writer - Time Synchronization Correction Mode"]
pub type CMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, STMR_SPEC, CMOD_A, O>;
impl<'a, const O: u8> CMOD_W<'a, O> {
    #[doc = "Mode 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMOD_A::_0)
    }
    #[doc = "Mode 2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMOD_A::_1)
    }
}
#[doc = "Field `W10S` reader - Worst 10 Acquisition Control Select"]
pub type W10S_R = crate::BitReader<W10S_A>;
#[doc = "Worst 10 Acquisition Control Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum W10S_A {
    #[doc = "0: Measurement is started by hardware and the value acquired in the PW10VR or MW10R register is used as the limit for filtering."]
    _0 = 0,
    #[doc = "1: Measurement is started by the GETW10R.GW10 bit. Also, the value set in the PLIMITR or MLIMITR register is used as the limit for filtering."]
    _1 = 1,
}
impl From<W10S_A> for bool {
    #[inline(always)]
    fn from(variant: W10S_A) -> Self {
        variant as u8 != 0
    }
}
impl W10S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> W10S_A {
        match self.bits {
            false => W10S_A::_0,
            true => W10S_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == W10S_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == W10S_A::_1
    }
}
#[doc = "Field `W10S` writer - Worst 10 Acquisition Control Select"]
pub type W10S_W<'a, const O: u8> = crate::BitWriter<'a, u32, STMR_SPEC, W10S_A, O>;
impl<'a, const O: u8> W10S_W<'a, O> {
    #[doc = "Measurement is started by hardware and the value acquired in the PW10VR or MW10R register is used as the limit for filtering."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(W10S_A::_0)
    }
    #[doc = "Measurement is started by the GETW10R.GW10 bit. Also, the value set in the PLIMITR or MLIMITR register is used as the limit for filtering."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(W10S_A::_1)
    }
}
#[doc = "Field `SYTH` reader - Synchronized State Detection Threshold Setting"]
pub type SYTH_R = crate::FieldReader<u8, SYTH_A>;
#[doc = "Synchronized State Detection Threshold Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYTH_A {
    #[doc = "0: None"]
    _0X0 = 0,
}
impl From<SYTH_A> for u8 {
    #[inline(always)]
    fn from(variant: SYTH_A) -> Self {
        variant as _
    }
}
impl SYTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYTH_A> {
        match self.bits {
            0 => Some(SYTH_A::_0X0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == SYTH_A::_0X0
    }
}
#[doc = "Field `SYTH` writer - Synchronized State Detection Threshold Setting"]
pub type SYTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STMR_SPEC, u8, SYTH_A, 4, O>;
impl<'a, const O: u8> SYTH_W<'a, O> {
    #[doc = "None"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(SYTH_A::_0X0)
    }
}
#[doc = "Field `DVTH` reader - Synchronization Loss Detection Threshold Setting"]
pub type DVTH_R = crate::FieldReader<u8, DVTH_A>;
#[doc = "Synchronization Loss Detection Threshold Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVTH_A {
    #[doc = "0: None"]
    _0X0 = 0,
}
impl From<DVTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DVTH_A) -> Self {
        variant as _
    }
}
impl DVTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DVTH_A> {
        match self.bits {
            0 => Some(DVTH_A::_0X0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == DVTH_A::_0X0
    }
}
#[doc = "Field `DVTH` writer - Synchronization Loss Detection Threshold Setting"]
pub type DVTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STMR_SPEC, u8, DVTH_A, 4, O>;
impl<'a, const O: u8> DVTH_W<'a, O> {
    #[doc = "None"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(DVTH_A::_0X0)
    }
}
#[doc = "Field `ALEN0` reader - Alarm Detection Enable 0"]
pub type ALEN0_R = crate::BitReader<ALEN0_A>;
#[doc = "Alarm Detection Enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALEN0_A {
    #[doc = "0: The STSR.SYNC or SYNCOUT flag is not set to 1 on detection of synchronization or loss of synchronization."]
    _0 = 0,
    #[doc = "1: The STSR.SYNC or SYNCOUT flag is set to 1 on detection of synchronization or loss of synchronization."]
    _1 = 1,
}
impl From<ALEN0_A> for bool {
    #[inline(always)]
    fn from(variant: ALEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl ALEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALEN0_A {
        match self.bits {
            false => ALEN0_A::_0,
            true => ALEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALEN0_A::_1
    }
}
#[doc = "Field `ALEN0` writer - Alarm Detection Enable 0"]
pub type ALEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STMR_SPEC, ALEN0_A, O>;
impl<'a, const O: u8> ALEN0_W<'a, O> {
    #[doc = "The STSR.SYNC or SYNCOUT flag is not set to 1 on detection of synchronization or loss of synchronization."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALEN0_A::_0)
    }
    #[doc = "The STSR.SYNC or SYNCOUT flag is set to 1 on detection of synchronization or loss of synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALEN0_A::_1)
    }
}
#[doc = "Field `ALEN1` reader - Alarm Detection Enable 1"]
pub type ALEN1_R = crate::BitReader<ALEN1_A>;
#[doc = "Alarm Detection Enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALEN1_A {
    #[doc = "0: The STSR.SYNTOUT flag is not set to 1 on detection of the Sync message reception timeout interrupt."]
    _0 = 0,
    #[doc = "1: The STSR.SYNTOUT flag is not set to 1 on detection of the Sync message reception timeout interrupt."]
    _1 = 1,
}
impl From<ALEN1_A> for bool {
    #[inline(always)]
    fn from(variant: ALEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl ALEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALEN1_A {
        match self.bits {
            false => ALEN1_A::_0,
            true => ALEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALEN1_A::_1
    }
}
#[doc = "Field `ALEN1` writer - Alarm Detection Enable 1"]
pub type ALEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STMR_SPEC, ALEN1_A, O>;
impl<'a, const O: u8> ALEN1_W<'a, O> {
    #[doc = "The STSR.SYNTOUT flag is not set to 1 on detection of the Sync message reception timeout interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALEN1_A::_0)
    }
    #[doc = "The STSR.SYNTOUT flag is not set to 1 on detection of the Sync message reception timeout interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALEN1_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Worst 10 Acquisition Time"]
    #[inline(always)]
    pub fn wint(&self) -> WINT_R {
        WINT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 13 - Time Synchronization Correction Mode"]
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Worst 10 Acquisition Control Select"]
    #[inline(always)]
    pub fn w10s(&self) -> W10S_R {
        W10S_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Synchronized State Detection Threshold Setting"]
    #[inline(always)]
    pub fn syth(&self) -> SYTH_R {
        SYTH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Synchronization Loss Detection Threshold Setting"]
    #[inline(always)]
    pub fn dvth(&self) -> DVTH_R {
        DVTH_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Alarm Detection Enable 0"]
    #[inline(always)]
    pub fn alen0(&self) -> ALEN0_R {
        ALEN0_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Alarm Detection Enable 1"]
    #[inline(always)]
    pub fn alen1(&self) -> ALEN1_R {
        ALEN1_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Worst 10 Acquisition Time"]
    #[inline(always)]
    #[must_use]
    pub fn wint(&mut self) -> WINT_W<0> {
        WINT_W::new(self)
    }
    #[doc = "Bit 13 - Time Synchronization Correction Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cmod(&mut self) -> CMOD_W<13> {
        CMOD_W::new(self)
    }
    #[doc = "Bit 15 - Worst 10 Acquisition Control Select"]
    #[inline(always)]
    #[must_use]
    pub fn w10s(&mut self) -> W10S_W<15> {
        W10S_W::new(self)
    }
    #[doc = "Bits 16:19 - Synchronized State Detection Threshold Setting"]
    #[inline(always)]
    #[must_use]
    pub fn syth(&mut self) -> SYTH_W<16> {
        SYTH_W::new(self)
    }
    #[doc = "Bits 20:23 - Synchronization Loss Detection Threshold Setting"]
    #[inline(always)]
    #[must_use]
    pub fn dvth(&mut self) -> DVTH_W<20> {
        DVTH_W::new(self)
    }
    #[doc = "Bit 28 - Alarm Detection Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn alen0(&mut self) -> ALEN0_W<28> {
        ALEN0_W::new(self)
    }
    #[doc = "Bit 29 - Alarm Detection Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn alen1(&mut self) -> ALEN1_W<29> {
        ALEN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "STCA Operating Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmr](index.html) module"]
pub struct STMR_SPEC;
impl crate::RegisterSpec for STMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stmr::R](R) reader structure"]
impl crate::Readable for STMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stmr::W](W) writer structure"]
impl crate::Writable for STMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STMR to value 0"]
impl crate::Resettable for STMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
