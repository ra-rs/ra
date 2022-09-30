#[doc = "Register `RDAYCNT` reader"]
pub struct R(crate::R<RDAYCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDAYCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDAYCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDAYCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RDAYCNT` writer"]
pub struct W(crate::W<RDAYCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RDAYCNT_SPEC>;
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
impl From<crate::W<RDAYCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RDAYCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATE1` reader - 1-Day Count"]
pub type DATE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATE1` writer - 1-Day Count"]
pub type DATE1_W<'a, const O: u8> = crate::FieldWriter<'a, u8, RDAYCNT_SPEC, u8, u8, 4, O>;
#[doc = "Field `DATE10` reader - 10-Day Count"]
pub type DATE10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATE10` writer - 10-Day Count"]
pub type DATE10_W<'a, const O: u8> = crate::FieldWriter<'a, u8, RDAYCNT_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:3 - 1-Day Count"]
    #[inline(always)]
    pub fn date1(&self) -> DATE1_R {
        DATE1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - 10-Day Count"]
    #[inline(always)]
    pub fn date10(&self) -> DATE10_R {
        DATE10_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-Day Count"]
    #[inline(always)]
    pub fn date1(&mut self) -> DATE1_W<0> {
        DATE1_W::new(self)
    }
    #[doc = "Bits 4:5 - 10-Day Count"]
    #[inline(always)]
    pub fn date10(&mut self) -> DATE10_W<4> {
        DATE10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Day Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdaycnt](index.html) module"]
pub struct RDAYCNT_SPEC;
impl crate::RegisterSpec for RDAYCNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rdaycnt::R](R) reader structure"]
impl crate::Readable for RDAYCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rdaycnt::W](W) writer structure"]
impl crate::Writable for RDAYCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RDAYCNT to value 0"]
impl crate::Resettable for RDAYCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
