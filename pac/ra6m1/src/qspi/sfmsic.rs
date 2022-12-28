#[doc = "Register `SFMSIC` reader"]
pub struct R(crate::R<SFMSIC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFMSIC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFMSIC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFMSIC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFMSIC` writer"]
pub struct W(crate::W<SFMSIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFMSIC_SPEC>;
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
impl From<crate::W<SFMSIC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFMSIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFMCIC` reader - Serial ROM instruction code to substitute"]
pub type SFMCIC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SFMCIC` writer - Serial ROM instruction code to substitute"]
pub type SFMCIC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SFMSIC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Serial ROM instruction code to substitute"]
    #[inline(always)]
    pub fn sfmcic(&self) -> SFMCIC_R {
        SFMCIC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Serial ROM instruction code to substitute"]
    #[inline(always)]
    #[must_use]
    pub fn sfmcic(&mut self) -> SFMCIC_W<0> {
        SFMCIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Instruction Code Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfmsic](index.html) module"]
pub struct SFMSIC_SPEC;
impl crate::RegisterSpec for SFMSIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfmsic::R](R) reader structure"]
impl crate::Readable for SFMSIC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfmsic::W](W) writer structure"]
impl crate::Writable for SFMSIC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFMSIC to value 0"]
impl crate::Resettable for SFMSIC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
