#[doc = "Register `SOFT_RST` reader"]
pub struct R(crate::R<SOFT_RST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOFT_RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOFT_RST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOFT_RST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOFT_RST` writer"]
pub struct W(crate::W<SOFT_RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOFT_RST_SPEC>;
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
impl From<crate::W<SOFT_RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOFT_RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDRST` reader - Software Reset of SD I/F Unit"]
pub type SDRST_R = crate::BitReader<SDRST_A>;
#[doc = "Software Reset of SD I/F Unit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDRST_A {
    #[doc = "0: Reset"]
    _0 = 0,
    #[doc = "1: Reset released"]
    _1 = 1,
}
impl From<SDRST_A> for bool {
    #[inline(always)]
    fn from(variant: SDRST_A) -> Self {
        variant as u8 != 0
    }
}
impl SDRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDRST_A {
        match self.bits {
            false => SDRST_A::_0,
            true => SDRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDRST_A::_1
    }
}
#[doc = "Field `SDRST` writer - Software Reset of SD I/F Unit"]
pub type SDRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOFT_RST_SPEC, SDRST_A, O>;
impl<'a, const O: u8> SDRST_W<'a, O> {
    #[doc = "Reset"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDRST_A::_0)
    }
    #[doc = "Reset released"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDRST_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset of SD I/F Unit"]
    #[inline(always)]
    pub fn sdrst(&self) -> SDRST_R {
        SDRST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset of SD I/F Unit"]
    #[inline(always)]
    #[must_use]
    pub fn sdrst(&mut self) -> SDRST_W<0> {
        SDRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soft_rst](index.html) module"]
pub struct SOFT_RST_SPEC;
impl crate::RegisterSpec for SOFT_RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [soft_rst::R](R) reader structure"]
impl crate::Readable for SOFT_RST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [soft_rst::W](W) writer structure"]
impl crate::Writable for SOFT_RST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOFT_RST to value 0x07"]
impl crate::Resettable for SOFT_RST_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
