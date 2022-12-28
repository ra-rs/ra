#[doc = "Register `DPUSR1R` reader"]
pub struct R(crate::R<DPUSR1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPUSR1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPUSR1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPUSR1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPUSR1R` writer"]
pub struct W(crate::W<DPUSR1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPUSR1R_SPEC>;
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
impl From<crate::W<DPUSR1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPUSR1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOVCAHE` reader - OVRCURA Interrupt Enable Clear"]
pub type DOVCAHE_R = crate::BitReader<DOVCAHE_A>;
#[doc = "OVRCURA Interrupt Enable Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOVCAHE_A {
    #[doc = "0: Disables return from deep software standby mode"]
    _0 = 0,
    #[doc = "1: Enables return from deep software standby mode"]
    _1 = 1,
}
impl From<DOVCAHE_A> for bool {
    #[inline(always)]
    fn from(variant: DOVCAHE_A) -> Self {
        variant as u8 != 0
    }
}
impl DOVCAHE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOVCAHE_A {
        match self.bits {
            false => DOVCAHE_A::_0,
            true => DOVCAHE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOVCAHE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOVCAHE_A::_1
    }
}
#[doc = "Field `DOVCAHE` writer - OVRCURA Interrupt Enable Clear"]
pub type DOVCAHE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPUSR1R_SPEC, DOVCAHE_A, O>;
impl<'a, const O: u8> DOVCAHE_W<'a, O> {
    #[doc = "Disables return from deep software standby mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOVCAHE_A::_0)
    }
    #[doc = "Enables return from deep software standby mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOVCAHE_A::_1)
    }
}
#[doc = "Field `DOVCBHE` reader - OVRCURB Interrupt Enable Clear"]
pub type DOVCBHE_R = crate::BitReader<DOVCBHE_A>;
#[doc = "OVRCURB Interrupt Enable Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOVCBHE_A {
    #[doc = "0: Disables return from deep software standby mode"]
    _0 = 0,
    #[doc = "1: Enables return from deep software standby mode"]
    _1 = 1,
}
impl From<DOVCBHE_A> for bool {
    #[inline(always)]
    fn from(variant: DOVCBHE_A) -> Self {
        variant as u8 != 0
    }
}
impl DOVCBHE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOVCBHE_A {
        match self.bits {
            false => DOVCBHE_A::_0,
            true => DOVCBHE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOVCBHE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOVCBHE_A::_1
    }
}
#[doc = "Field `DOVCBHE` writer - OVRCURB Interrupt Enable Clear"]
pub type DOVCBHE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPUSR1R_SPEC, DOVCBHE_A, O>;
impl<'a, const O: u8> DOVCBHE_W<'a, O> {
    #[doc = "Disables return from deep software standby mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOVCBHE_A::_0)
    }
    #[doc = "Enables return from deep software standby mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOVCBHE_A::_1)
    }
}
#[doc = "Field `DVBSTSHE` reader - VBUS Interrupt Enable/Clear"]
pub type DVBSTSHE_R = crate::BitReader<DVBSTSHE_A>;
#[doc = "VBUS Interrupt Enable/Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVBSTSHE_A {
    #[doc = "0: Disables return from deep software standby mode"]
    _0 = 0,
    #[doc = "1: Enables return from deep software standby mode"]
    _1 = 1,
}
impl From<DVBSTSHE_A> for bool {
    #[inline(always)]
    fn from(variant: DVBSTSHE_A) -> Self {
        variant as u8 != 0
    }
}
impl DVBSTSHE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVBSTSHE_A {
        match self.bits {
            false => DVBSTSHE_A::_0,
            true => DVBSTSHE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DVBSTSHE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DVBSTSHE_A::_1
    }
}
#[doc = "Field `DVBSTSHE` writer - VBUS Interrupt Enable/Clear"]
pub type DVBSTSHE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPUSR1R_SPEC, DVBSTSHE_A, O>;
impl<'a, const O: u8> DVBSTSHE_W<'a, O> {
    #[doc = "Disables return from deep software standby mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DVBSTSHE_A::_0)
    }
    #[doc = "Enables return from deep software standby mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DVBSTSHE_A::_1)
    }
}
#[doc = "Field `DOVCAH` reader - Indication of Return from OVRCURA Interrupt Source"]
pub type DOVCAH_R = crate::BitReader<DOVCAH_A>;
#[doc = "Indication of Return from OVRCURA Interrupt Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOVCAH_A {
    #[doc = "0: Indicates deep software standby mode"]
    _0 = 0,
    #[doc = "1: Indicates return from deep software standby mode"]
    _1 = 1,
}
impl From<DOVCAH_A> for bool {
    #[inline(always)]
    fn from(variant: DOVCAH_A) -> Self {
        variant as u8 != 0
    }
}
impl DOVCAH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOVCAH_A {
        match self.bits {
            false => DOVCAH_A::_0,
            true => DOVCAH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOVCAH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOVCAH_A::_1
    }
}
#[doc = "Field `DOVCBH` reader - Indication of Return from OVRCURB Interrupt Source"]
pub type DOVCBH_R = crate::BitReader<DOVCBH_A>;
#[doc = "Indication of Return from OVRCURB Interrupt Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOVCBH_A {
    #[doc = "0: Indicates deep software standby mode"]
    _0 = 0,
    #[doc = "1: Indicates return from deep software standby mode"]
    _1 = 1,
}
impl From<DOVCBH_A> for bool {
    #[inline(always)]
    fn from(variant: DOVCBH_A) -> Self {
        variant as u8 != 0
    }
}
impl DOVCBH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOVCBH_A {
        match self.bits {
            false => DOVCBH_A::_0,
            true => DOVCBH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOVCBH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOVCBH_A::_1
    }
}
#[doc = "Field `DVBSTSH` reader - Indication of Return from VBUS Interrupt Source"]
pub type DVBSTSH_R = crate::BitReader<DVBSTSH_A>;
#[doc = "Indication of Return from VBUS Interrupt Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVBSTSH_A {
    #[doc = "0: Indicates deep software standby mode"]
    _0 = 0,
    #[doc = "1: Indicates return from deep software standby mode"]
    _1 = 1,
}
impl From<DVBSTSH_A> for bool {
    #[inline(always)]
    fn from(variant: DVBSTSH_A) -> Self {
        variant as u8 != 0
    }
}
impl DVBSTSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVBSTSH_A {
        match self.bits {
            false => DVBSTSH_A::_0,
            true => DVBSTSH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DVBSTSH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DVBSTSH_A::_1
    }
}
impl R {
    #[doc = "Bit 4 - OVRCURA Interrupt Enable Clear"]
    #[inline(always)]
    pub fn dovcahe(&self) -> DOVCAHE_R {
        DOVCAHE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OVRCURB Interrupt Enable Clear"]
    #[inline(always)]
    pub fn dovcbhe(&self) -> DOVCBHE_R {
        DOVCBHE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - VBUS Interrupt Enable/Clear"]
    #[inline(always)]
    pub fn dvbstshe(&self) -> DVBSTSHE_R {
        DVBSTSHE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 20 - Indication of Return from OVRCURA Interrupt Source"]
    #[inline(always)]
    pub fn dovcah(&self) -> DOVCAH_R {
        DOVCAH_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Indication of Return from OVRCURB Interrupt Source"]
    #[inline(always)]
    pub fn dovcbh(&self) -> DOVCBH_R {
        DOVCBH_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Indication of Return from VBUS Interrupt Source"]
    #[inline(always)]
    pub fn dvbstsh(&self) -> DVBSTSH_R {
        DVBSTSH_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - OVRCURA Interrupt Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dovcahe(&mut self) -> DOVCAHE_W<4> {
        DOVCAHE_W::new(self)
    }
    #[doc = "Bit 5 - OVRCURB Interrupt Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dovcbhe(&mut self) -> DOVCBHE_W<5> {
        DOVCBHE_W::new(self)
    }
    #[doc = "Bit 7 - VBUS Interrupt Enable/Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dvbstshe(&mut self) -> DVBSTSHE_W<7> {
        DVBSTSHE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep Standby USB Suspend/Resume Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpusr1r](index.html) module"]
pub struct DPUSR1R_SPEC;
impl crate::RegisterSpec for DPUSR1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpusr1r::R](R) reader structure"]
impl crate::Readable for DPUSR1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpusr1r::W](W) writer structure"]
impl crate::Writable for DPUSR1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPUSR1R to value 0"]
impl crate::Resettable for DPUSR1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
