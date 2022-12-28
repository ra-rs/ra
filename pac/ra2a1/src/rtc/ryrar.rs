#[doc = "Register `RYRAR` reader"]
pub struct R(crate::R<RYRAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RYRAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RYRAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RYRAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RYRAR` writer"]
pub struct W(crate::W<RYRAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RYRAR_SPEC>;
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
impl From<crate::W<RYRAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RYRAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `YR1` reader - 1 Year Value for the ones place of years"]
pub type YR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YR1` writer - 1 Year Value for the ones place of years"]
pub type YR1_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RYRAR_SPEC, u8, u8, 4, O>;
#[doc = "Field `YR10` reader - 10 Years Value for the tens place of years"]
pub type YR10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YR10` writer - 10 Years Value for the tens place of years"]
pub type YR10_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RYRAR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 1 Year Value for the ones place of years"]
    #[inline(always)]
    pub fn yr1(&self) -> YR1_R {
        YR1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 10 Years Value for the tens place of years"]
    #[inline(always)]
    pub fn yr10(&self) -> YR10_R {
        YR10_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1 Year Value for the ones place of years"]
    #[inline(always)]
    #[must_use]
    pub fn yr1(&mut self) -> YR1_W<0> {
        YR1_W::new(self)
    }
    #[doc = "Bits 4:7 - 10 Years Value for the tens place of years"]
    #[inline(always)]
    #[must_use]
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
#[doc = "Year Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ryrar](index.html) module"]
pub struct RYRAR_SPEC;
impl crate::RegisterSpec for RYRAR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ryrar::R](R) reader structure"]
impl crate::Readable for RYRAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ryrar::W](W) writer structure"]
impl crate::Writable for RYRAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RYRAR to value 0"]
impl crate::Resettable for RYRAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
