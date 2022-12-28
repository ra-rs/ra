#[doc = "Register `BCCTRL2` reader"]
pub struct R(crate::R<BCCTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCCTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCCTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCCTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCCTRL2` writer"]
pub struct W(crate::W<BCCTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCCTRL2_SPEC>;
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
impl From<crate::W<BCCTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCCTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCPMODE` reader - Dedicated Charging Port (DCP) Mode Control"]
pub type DCPMODE_R = crate::BitReader<DCPMODE_A>;
#[doc = "Dedicated Charging Port (DCP) Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCPMODE_A {
    #[doc = "0: Disable DCP"]
    _0 = 0,
    #[doc = "1: Enable DCP"]
    _1 = 1,
}
impl From<DCPMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DCPMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl DCPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCPMODE_A {
        match self.bits {
            false => DCPMODE_A::_0,
            true => DCPMODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCPMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCPMODE_A::_1
    }
}
#[doc = "Field `DCPMODE` writer - Dedicated Charging Port (DCP) Mode Control"]
pub type DCPMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCCTRL2_SPEC, DCPMODE_A, O>;
impl<'a, const O: u8> DCPMODE_W<'a, O> {
    #[doc = "Disable DCP"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCPMODE_A::_0)
    }
    #[doc = "Enable DCP"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCPMODE_A::_1)
    }
}
#[doc = "Field `BATCHGE` reader - Battery Charging Enable"]
pub type BATCHGE_R = crate::BitReader<BATCHGE_A>;
#[doc = "Battery Charging Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BATCHGE_A {
    #[doc = "0: Disable Battery Charging"]
    _0 = 0,
    #[doc = "1: Enable Battery Charging"]
    _1 = 1,
}
impl From<BATCHGE_A> for bool {
    #[inline(always)]
    fn from(variant: BATCHGE_A) -> Self {
        variant as u8 != 0
    }
}
impl BATCHGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BATCHGE_A {
        match self.bits {
            false => BATCHGE_A::_0,
            true => BATCHGE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BATCHGE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BATCHGE_A::_1
    }
}
#[doc = "Field `BATCHGE` writer - Battery Charging Enable"]
pub type BATCHGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCCTRL2_SPEC, BATCHGE_A, O>;
impl<'a, const O: u8> BATCHGE_W<'a, O> {
    #[doc = "Disable Battery Charging"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BATCHGE_A::_0)
    }
    #[doc = "Enable Battery Charging"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BATCHGE_A::_1)
    }
}
#[doc = "Field `PHYDET` reader - Detect Sensitivity Adjustment"]
pub type PHYDET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PHYDET` writer - Detect Sensitivity Adjustment"]
pub type PHYDET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCCTRL2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 6 - Dedicated Charging Port (DCP) Mode Control"]
    #[inline(always)]
    pub fn dcpmode(&self) -> DCPMODE_R {
        DCPMODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Battery Charging Enable"]
    #[inline(always)]
    pub fn batchge(&self) -> BATCHGE_R {
        BATCHGE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Detect Sensitivity Adjustment"]
    #[inline(always)]
    pub fn phydet(&self) -> PHYDET_R {
        PHYDET_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 6 - Dedicated Charging Port (DCP) Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn dcpmode(&mut self) -> DCPMODE_W<6> {
        DCPMODE_W::new(self)
    }
    #[doc = "Bit 7 - Battery Charging Enable"]
    #[inline(always)]
    #[must_use]
    pub fn batchge(&mut self) -> BATCHGE_W<7> {
        BATCHGE_W::new(self)
    }
    #[doc = "Bits 12:13 - Detect Sensitivity Adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn phydet(&mut self) -> PHYDET_W<12> {
        PHYDET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Battery Charging Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcctrl2](index.html) module"]
pub struct BCCTRL2_SPEC;
impl crate::RegisterSpec for BCCTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bcctrl2::R](R) reader structure"]
impl crate::Readable for BCCTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcctrl2::W](W) writer structure"]
impl crate::Writable for BCCTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCCTRL2 to value 0x2000"]
impl crate::Resettable for BCCTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000;
}
