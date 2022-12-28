#[doc = "Register `MB%s_D4` reader"]
pub struct R(crate::R<MB_D4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MB_D4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MB_D4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MB_D4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MB%s_D4` writer"]
pub struct W(crate::W<MB_D4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MB_D4_SPEC>;
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
impl From<crate::W<MB_D4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MB_D4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA4` reader - Data Bytes 4"]
pub type DATA4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA4` writer - Data Bytes 4"]
pub type DATA4_W<'a, const O: u8> = crate::FieldWriter<'a, u8, MB_D4_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data Bytes 4"]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Bytes 4"]
    #[inline(always)]
    #[must_use]
    pub fn data4(&mut self) -> DATA4_W<0> {
        DATA4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mailbox Data Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mb_d4](index.html) module"]
pub struct MB_D4_SPEC;
impl crate::RegisterSpec for MB_D4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mb_d4::R](R) reader structure"]
impl crate::Readable for MB_D4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mb_d4::W](W) writer structure"]
impl crate::Writable for MB_D4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MB%s_D4 to value 0"]
impl crate::Resettable for MB_D4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
