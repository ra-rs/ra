#[doc = "Register `CR3` reader"]
pub struct R(crate::R<CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR3` writer"]
pub struct W(crate::W<CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR3_SPEC>;
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
impl From<crate::W<CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDST` reader - Start Frame Detection Start"]
pub type SDST_R = crate::BitReader<SDST_A>;
#[doc = "Start Frame Detection Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDST_A {
    #[doc = "0: Detection of Start Frame is not performed."]
    _0 = 0,
    #[doc = "1: Detection of Start Frame is performed."]
    _1 = 1,
}
impl From<SDST_A> for bool {
    #[inline(always)]
    fn from(variant: SDST_A) -> Self {
        variant as u8 != 0
    }
}
impl SDST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDST_A {
        match self.bits {
            false => SDST_A::_0,
            true => SDST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDST_A::_1
    }
}
#[doc = "Field `SDST` writer - Start Frame Detection Start"]
pub type SDST_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR3_SPEC, SDST_A, O>;
impl<'a, const O: u8> SDST_W<'a, O> {
    #[doc = "Detection of Start Frame is not performed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDST_A::_0)
    }
    #[doc = "Detection of Start Frame is performed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDST_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Start Frame Detection Start"]
    #[inline(always)]
    pub fn sdst(&self) -> SDST_R {
        SDST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Frame Detection Start"]
    #[inline(always)]
    #[must_use]
    pub fn sdst(&mut self) -> SDST_W<0> {
        SDST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr3](index.html) module"]
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cr3::R](R) reader structure"]
impl crate::Readable for CR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr3::W](W) writer structure"]
impl crate::Writable for CR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for CR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
