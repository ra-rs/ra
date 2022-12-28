#[doc = "Register `EC710TED` reader"]
pub struct R(crate::R<EC710TED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EC710TED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EC710TED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EC710TED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EC710TED` writer"]
pub struct W(crate::W<EC710TED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EC710TED_SPEC>;
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
impl From<crate::W<EC710TED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EC710TED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECEDB` reader - ECC Test Substitute Data"]
pub type ECEDB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ECEDB` writer - ECC Test Substitute Data"]
pub type ECEDB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EC710TED_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - ECC Test Substitute Data"]
    #[inline(always)]
    pub fn ecedb(&self) -> ECEDB_R {
        ECEDB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ECC Test Substitute Data"]
    #[inline(always)]
    #[must_use]
    pub fn ecedb(&mut self) -> ECEDB_W<0> {
        ECEDB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECC Test Substitute Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ec710ted](index.html) module"]
pub struct EC710TED_SPEC;
impl crate::RegisterSpec for EC710TED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ec710ted::R](R) reader structure"]
impl crate::Readable for EC710TED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ec710ted::W](W) writer structure"]
impl crate::Writable for EC710TED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EC710TED to value 0"]
impl crate::Resettable for EC710TED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
