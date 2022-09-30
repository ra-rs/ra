#[doc = "Register `RYRCNT` reader"]
pub struct R(crate::R<RYRCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RYRCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RYRCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RYRCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RYRCNT` writer"]
pub struct W(crate::W<RYRCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RYRCNT_SPEC>;
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
impl From<crate::W<RYRCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RYRCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `YR1` reader - 1-Year Count"]
pub type YR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YR1` writer - 1-Year Count"]
pub type YR1_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RYRCNT_SPEC, u8, u8, 4, O>;
#[doc = "Field `YR10` reader - 10-Year Count"]
pub type YR10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YR10` writer - 10-Year Count"]
pub type YR10_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RYRCNT_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 1-Year Count"]
    #[inline(always)]
    pub fn yr1(&self) -> YR1_R {
        YR1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 10-Year Count"]
    #[inline(always)]
    pub fn yr10(&self) -> YR10_R {
        YR10_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-Year Count"]
    #[inline(always)]
    pub fn yr1(&mut self) -> YR1_W<0> {
        YR1_W::new(self)
    }
    #[doc = "Bits 4:7 - 10-Year Count"]
    #[inline(always)]
    pub fn yr10(&mut self) -> YR10_W<4> {
        YR10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Year Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ryrcnt](index.html) module"]
pub struct RYRCNT_SPEC;
impl crate::RegisterSpec for RYRCNT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ryrcnt::R](R) reader structure"]
impl crate::Readable for RYRCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ryrcnt::W](W) writer structure"]
impl crate::Writable for RYRCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RYRCNT to value 0"]
impl crate::Resettable for RYRCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
