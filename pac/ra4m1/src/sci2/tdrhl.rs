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
#[doc = "Field `TDRHL` writer - TDRHL is a 16-bit register that stores transmit data."]
pub type TDRHL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TDRHL_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15 - TDRHL is a 16-bit register that stores transmit data."]
    #[inline(always)]
    #[must_use]
    pub fn tdrhl(&mut self) -> TDRHL_W<0> {
        TDRHL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit 9-bit Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdrhl](index.html) module"]
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
