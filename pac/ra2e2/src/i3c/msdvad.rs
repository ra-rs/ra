#[doc = "Register `MSDVAD` reader"]
pub struct R(crate::R<MSDVAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSDVAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSDVAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSDVAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSDVAD` writer"]
pub struct W(crate::W<MSDVAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSDVAD_SPEC>;
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
impl From<crate::W<MSDVAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSDVAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDYAD` reader - Master Dynamic Address"]
pub type MDYAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MDYAD` writer - Master Dynamic Address"]
pub type MDYAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MSDVAD_SPEC, u8, u8, 7, O>;
#[doc = "Field `MDYADV` reader - Master Dynamic Address Valid"]
pub type MDYADV_R = crate::BitReader<MDYADV_A>;
#[doc = "Master Dynamic Address Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDYADV_A {
    #[doc = "0: The master dynamic address field is not valid."]
    _0 = 0,
    #[doc = "1: The master dynamic address field is valid."]
    _1 = 1,
}
impl From<MDYADV_A> for bool {
    #[inline(always)]
    fn from(variant: MDYADV_A) -> Self {
        variant as u8 != 0
    }
}
impl MDYADV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDYADV_A {
        match self.bits {
            false => MDYADV_A::_0,
            true => MDYADV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MDYADV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MDYADV_A::_1
    }
}
#[doc = "Field `MDYADV` writer - Master Dynamic Address Valid"]
pub type MDYADV_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSDVAD_SPEC, MDYADV_A, O>;
impl<'a, const O: u8> MDYADV_W<'a, O> {
    #[doc = "The master dynamic address field is not valid."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MDYADV_A::_0)
    }
    #[doc = "The master dynamic address field is valid."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MDYADV_A::_1)
    }
}
impl R {
    #[doc = "Bits 16:22 - Master Dynamic Address"]
    #[inline(always)]
    pub fn mdyad(&self) -> MDYAD_R {
        MDYAD_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Master Dynamic Address Valid"]
    #[inline(always)]
    pub fn mdyadv(&self) -> MDYADV_R {
        MDYADV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:22 - Master Dynamic Address"]
    #[inline(always)]
    #[must_use]
    pub fn mdyad(&mut self) -> MDYAD_W<16> {
        MDYAD_W::new(self)
    }
    #[doc = "Bit 31 - Master Dynamic Address Valid"]
    #[inline(always)]
    #[must_use]
    pub fn mdyadv(&mut self) -> MDYADV_W<31> {
        MDYADV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Device Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msdvad](index.html) module"]
pub struct MSDVAD_SPEC;
impl crate::RegisterSpec for MSDVAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msdvad::R](R) reader structure"]
impl crate::Readable for MSDVAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msdvad::W](W) writer structure"]
impl crate::Writable for MSDVAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSDVAD to value 0"]
impl crate::Resettable for MSDVAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
