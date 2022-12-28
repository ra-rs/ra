#[doc = "Register `VBTSR` reader"]
pub struct R(crate::R<VBTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VBTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VBTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VBTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VBTSR` writer"]
pub struct W(crate::W<VBTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VBTSR_SPEC>;
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
impl From<crate::W<VBTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VBTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBTRDF` reader - VBAT_R Reset Detect Flag\n\nThe field is **modified** in some way after a read operation."]
pub type VBTRDF_R = crate::BitReader<VBTRDF_A>;
#[doc = "VBAT_R Reset Detect Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBTRDF_A {
    #[doc = "0: VBATT_R voltage power-on reset not detected"]
    _0 = 0,
    #[doc = "1: VBATT_R selected voltage power-on reset detected."]
    _1 = 1,
}
impl From<VBTRDF_A> for bool {
    #[inline(always)]
    fn from(variant: VBTRDF_A) -> Self {
        variant as u8 != 0
    }
}
impl VBTRDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBTRDF_A {
        match self.bits {
            false => VBTRDF_A::_0,
            true => VBTRDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBTRDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBTRDF_A::_1
    }
}
#[doc = "Field `VBTRDF` writer - VBAT_R Reset Detect Flag"]
pub type VBTRDF_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, VBTSR_SPEC, VBTRDF_A, O>;
impl<'a, const O: u8> VBTRDF_W<'a, O> {
    #[doc = "VBATT_R voltage power-on reset not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VBTRDF_A::_0)
    }
    #[doc = "VBATT_R selected voltage power-on reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VBTRDF_A::_1)
    }
}
#[doc = "Field `VBTBLDF` reader - VBATT Battery Low voltage Detect Flag\n\nThe field is **modified** in some way after a read operation."]
pub type VBTBLDF_R = crate::BitReader<VBTBLDF_A>;
#[doc = "VBATT Battery Low voltage Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBTBLDF_A {
    #[doc = "0: VBATT pin low voltage not detected"]
    _0 = 0,
    #[doc = "1: VBATT pin low voltage detected."]
    _1 = 1,
}
impl From<VBTBLDF_A> for bool {
    #[inline(always)]
    fn from(variant: VBTBLDF_A) -> Self {
        variant as u8 != 0
    }
}
impl VBTBLDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBTBLDF_A {
        match self.bits {
            false => VBTBLDF_A::_0,
            true => VBTBLDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBTBLDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBTBLDF_A::_1
    }
}
#[doc = "Field `VBTBLDF` writer - VBATT Battery Low voltage Detect Flag"]
pub type VBTBLDF_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, VBTSR_SPEC, VBTBLDF_A, O>;
impl<'a, const O: u8> VBTBLDF_W<'a, O> {
    #[doc = "VBATT pin low voltage not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VBTBLDF_A::_0)
    }
    #[doc = "VBATT pin low voltage detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VBTBLDF_A::_1)
    }
}
#[doc = "Field `VBTRVLD` reader - VBATT_R Valid"]
pub type VBTRVLD_R = crate::BitReader<VBTRVLD_A>;
#[doc = "VBATT_R Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBTRVLD_A {
    #[doc = "0: VBATT_R area not valid"]
    _0 = 0,
    #[doc = "1: VBATT_R area valid"]
    _1 = 1,
}
impl From<VBTRVLD_A> for bool {
    #[inline(always)]
    fn from(variant: VBTRVLD_A) -> Self {
        variant as u8 != 0
    }
}
impl VBTRVLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBTRVLD_A {
        match self.bits {
            false => VBTRVLD_A::_0,
            true => VBTRVLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBTRVLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBTRVLD_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - VBAT_R Reset Detect Flag"]
    #[inline(always)]
    pub fn vbtrdf(&self) -> VBTRDF_R {
        VBTRDF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBATT Battery Low voltage Detect Flag"]
    #[inline(always)]
    pub fn vbtbldf(&self) -> VBTBLDF_R {
        VBTBLDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - VBATT_R Valid"]
    #[inline(always)]
    pub fn vbtrvld(&self) -> VBTRVLD_R {
        VBTRVLD_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBAT_R Reset Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vbtrdf(&mut self) -> VBTRDF_W<0> {
        VBTRDF_W::new(self)
    }
    #[doc = "Bit 1 - VBATT Battery Low voltage Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vbtbldf(&mut self) -> VBTBLDF_W<1> {
        VBTBLDF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VBATT Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vbtsr](index.html) module"]
pub struct VBTSR_SPEC;
impl crate::RegisterSpec for VBTSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vbtsr::R](R) reader structure"]
impl crate::Readable for VBTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vbtsr::W](W) writer structure"]
impl crate::Writable for VBTSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x03;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VBTSR to value 0x01"]
impl crate::Resettable for VBTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
