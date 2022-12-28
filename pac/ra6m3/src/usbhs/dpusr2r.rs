#[doc = "Register `DPUSR2R` reader"]
pub struct R(crate::R<DPUSR2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPUSR2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPUSR2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPUSR2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPUSR2R` writer"]
pub struct W(crate::W<DPUSR2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPUSR2R_SPEC>;
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
impl From<crate::W<DPUSR2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPUSR2R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DPINT` reader - Indication of Return from DP Interrupt Source"]
pub type DPINT_R = crate::BitReader<DPINT_A>;
#[doc = "Indication of Return from DP Interrupt Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPINT_A {
    #[doc = "0: Indicates deep software standby mode"]
    _0 = 0,
    #[doc = "1: Indicates return from deep software standby mode"]
    _1 = 1,
}
impl From<DPINT_A> for bool {
    #[inline(always)]
    fn from(variant: DPINT_A) -> Self {
        variant as u8 != 0
    }
}
impl DPINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPINT_A {
        match self.bits {
            false => DPINT_A::_0,
            true => DPINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPINT_A::_1
    }
}
#[doc = "Field `DMINT` reader - Indication of Return from DM Interrupt Source"]
pub type DMINT_R = crate::BitReader<DMINT_A>;
#[doc = "Indication of Return from DM Interrupt Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMINT_A {
    #[doc = "0: Indicates deep software standby mode"]
    _0 = 0,
    #[doc = "1: Indicates return from deep software standby mode"]
    _1 = 1,
}
impl From<DMINT_A> for bool {
    #[inline(always)]
    fn from(variant: DMINT_A) -> Self {
        variant as u8 != 0
    }
}
impl DMINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMINT_A {
        match self.bits {
            false => DMINT_A::_0,
            true => DMINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMINT_A::_1
    }
}
#[doc = "Field `DPVAL` reader - DP InputIndicates DP input signal on the HS side of USB port."]
pub type DPVAL_R = crate::BitReader<bool>;
#[doc = "Field `DMVAL` reader - DM InputIndicates DM input signal on the HS side of USB port."]
pub type DMVAL_R = crate::BitReader<bool>;
#[doc = "Field `DPINTE` reader - DP Interrupt Enable Clear"]
pub type DPINTE_R = crate::BitReader<DPINTE_A>;
#[doc = "DP Interrupt Enable Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPINTE_A {
    #[doc = "0: Disables return from deep software standby mode"]
    _0 = 0,
    #[doc = "1: Enables return from deep software standby mode"]
    _1 = 1,
}
impl From<DPINTE_A> for bool {
    #[inline(always)]
    fn from(variant: DPINTE_A) -> Self {
        variant as u8 != 0
    }
}
impl DPINTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPINTE_A {
        match self.bits {
            false => DPINTE_A::_0,
            true => DPINTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPINTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPINTE_A::_1
    }
}
#[doc = "Field `DPINTE` writer - DP Interrupt Enable Clear"]
pub type DPINTE_W<'a, const O: u8> = crate::BitWriter<'a, u16, DPUSR2R_SPEC, DPINTE_A, O>;
impl<'a, const O: u8> DPINTE_W<'a, O> {
    #[doc = "Disables return from deep software standby mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPINTE_A::_0)
    }
    #[doc = "Enables return from deep software standby mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPINTE_A::_1)
    }
}
#[doc = "Field `DMINTE` reader - DM Interrupt Enable Clear"]
pub type DMINTE_R = crate::BitReader<DMINTE_A>;
#[doc = "DM Interrupt Enable Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMINTE_A {
    #[doc = "0: Disables return from deep software standby mode"]
    _0 = 0,
    #[doc = "1: Enables return from deep software standby mode"]
    _1 = 1,
}
impl From<DMINTE_A> for bool {
    #[inline(always)]
    fn from(variant: DMINTE_A) -> Self {
        variant as u8 != 0
    }
}
impl DMINTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMINTE_A {
        match self.bits {
            false => DMINTE_A::_0,
            true => DMINTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMINTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMINTE_A::_1
    }
}
#[doc = "Field `DMINTE` writer - DM Interrupt Enable Clear"]
pub type DMINTE_W<'a, const O: u8> = crate::BitWriter<'a, u16, DPUSR2R_SPEC, DMINTE_A, O>;
impl<'a, const O: u8> DMINTE_W<'a, O> {
    #[doc = "Disables return from deep software standby mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMINTE_A::_0)
    }
    #[doc = "Enables return from deep software standby mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMINTE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Indication of Return from DP Interrupt Source"]
    #[inline(always)]
    pub fn dpint(&self) -> DPINT_R {
        DPINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indication of Return from DM Interrupt Source"]
    #[inline(always)]
    pub fn dmint(&self) -> DMINT_R {
        DMINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - DP InputIndicates DP input signal on the HS side of USB port."]
    #[inline(always)]
    pub fn dpval(&self) -> DPVAL_R {
        DPVAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DM InputIndicates DM input signal on the HS side of USB port."]
    #[inline(always)]
    pub fn dmval(&self) -> DMVAL_R {
        DMVAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - DP Interrupt Enable Clear"]
    #[inline(always)]
    pub fn dpinte(&self) -> DPINTE_R {
        DPINTE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DM Interrupt Enable Clear"]
    #[inline(always)]
    pub fn dminte(&self) -> DMINTE_R {
        DMINTE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - DP Interrupt Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dpinte(&mut self) -> DPINTE_W<8> {
        DPINTE_W::new(self)
    }
    #[doc = "Bit 9 - DM Interrupt Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dminte(&mut self) -> DMINTE_W<9> {
        DMINTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep Standby USB Suspend/Resume Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpusr2r](index.html) module"]
pub struct DPUSR2R_SPEC;
impl crate::RegisterSpec for DPUSR2R_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dpusr2r::R](R) reader structure"]
impl crate::Readable for DPUSR2R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpusr2r::W](W) writer structure"]
impl crate::Writable for DPUSR2R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPUSR2R to value 0"]
impl crate::Resettable for DPUSR2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
