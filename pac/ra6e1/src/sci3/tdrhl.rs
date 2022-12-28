#[doc = "Register `TDRHL` reader"]
pub struct R(crate::R<TDRHL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TDRHL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TDRHL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TDRHL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TDRHL` writer"]
pub struct W(crate::W<TDRHL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TDRHL_SPEC>;
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
impl From<crate::W<TDRHL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TDRHL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDAT` reader - Serial Transmit Data"]
pub type TDAT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TDAT` writer - Serial Transmit Data"]
pub type TDAT_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TDRHL_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - Serial Transmit Data"]
    #[inline(always)]
    pub fn tdat(&self) -> TDAT_R {
        TDAT_R::new(self.bits & 0x01ff)
    }
}
impl W {
    #[doc = "Bits 0:8 - Serial Transmit Data"]
    #[inline(always)]
    #[must_use]
    pub fn tdat(&mut self) -> TDAT_W<0> {
        TDAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Data Register for Non-Manchester mode (MMR.MANEN = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdrhl](index.html) module"]
pub struct TDRHL_SPEC;
impl crate::RegisterSpec for TDRHL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tdrhl::R](R) reader structure"]
impl crate::Readable for TDRHL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tdrhl::W](W) writer structure"]
impl crate::Writable for TDRHL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TDRHL to value 0xffff"]
impl crate::Resettable for TDRHL_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
