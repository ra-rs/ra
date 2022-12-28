#[doc = "Register `CFDGAFLECTR` reader"]
pub struct R(crate::R<CFDGAFLECTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDGAFLECTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDGAFLECTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDGAFLECTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDGAFLECTR` writer"]
pub struct W(crate::W<CFDGAFLECTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDGAFLECTR_SPEC>;
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
impl From<crate::W<CFDGAFLECTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDGAFLECTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFLPN` reader - Acceptance Filter List Page Number"]
pub type AFLPN_R = crate::BitReader<bool>;
#[doc = "Field `AFLPN` writer - Acceptance Filter List Page Number"]
pub type AFLPN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGAFLECTR_SPEC, bool, O>;
#[doc = "Field `AFLDAE` reader - Acceptance Filter List Data Access Enable"]
pub type AFLDAE_R = crate::BitReader<AFLDAE_A>;
#[doc = "Acceptance Filter List Data Access Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFLDAE_A {
    #[doc = "0: Acceptance Filter List data access disabled"]
    _0 = 0,
    #[doc = "1: Acceptance Filter List data access enabled"]
    _1 = 1,
}
impl From<AFLDAE_A> for bool {
    #[inline(always)]
    fn from(variant: AFLDAE_A) -> Self {
        variant as u8 != 0
    }
}
impl AFLDAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AFLDAE_A {
        match self.bits {
            false => AFLDAE_A::_0,
            true => AFLDAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AFLDAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AFLDAE_A::_1
    }
}
#[doc = "Field `AFLDAE` writer - Acceptance Filter List Data Access Enable"]
pub type AFLDAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGAFLECTR_SPEC, AFLDAE_A, O>;
impl<'a, const O: u8> AFLDAE_W<'a, O> {
    #[doc = "Acceptance Filter List data access disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AFLDAE_A::_0)
    }
    #[doc = "Acceptance Filter List data access enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AFLDAE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Acceptance Filter List Page Number"]
    #[inline(always)]
    pub fn aflpn(&self) -> AFLPN_R {
        AFLPN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Acceptance Filter List Data Access Enable"]
    #[inline(always)]
    pub fn afldae(&self) -> AFLDAE_R {
        AFLDAE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Acceptance Filter List Page Number"]
    #[inline(always)]
    #[must_use]
    pub fn aflpn(&mut self) -> AFLPN_W<0> {
        AFLPN_W::new(self)
    }
    #[doc = "Bit 8 - Acceptance Filter List Data Access Enable"]
    #[inline(always)]
    #[must_use]
    pub fn afldae(&mut self) -> AFLDAE_W<8> {
        AFLDAE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Acceptance Filter List Entry Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdgaflectr](index.html) module"]
pub struct CFDGAFLECTR_SPEC;
impl crate::RegisterSpec for CFDGAFLECTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdgaflectr::R](R) reader structure"]
impl crate::Readable for CFDGAFLECTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdgaflectr::W](W) writer structure"]
impl crate::Writable for CFDGAFLECTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDGAFLECTR to value 0"]
impl crate::Resettable for CFDGAFLECTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
