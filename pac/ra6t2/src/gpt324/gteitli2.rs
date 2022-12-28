#[doc = "Register `GTEITLI2` reader"]
pub struct R(crate::R<GTEITLI2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTEITLI2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTEITLI2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTEITLI2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTEITLI2` writer"]
pub struct W(crate::W<GTEITLI2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTEITLI2_SPEC>;
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
impl From<crate::W<GTEITLI2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTEITLI2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EADTAL` reader - GTADTRA Register A/D Conversion Start Request Extended Skipping Function Select"]
pub type EADTAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EADTAL` writer - GTADTRA Register A/D Conversion Start Request Extended Skipping Function Select"]
pub type EADTAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTEITLI2_SPEC, u8, u8, 3, O>;
#[doc = "Field `EADTBL` reader - GTADTRB Register A/D Conversion Start Request Extended Skipping Function Select"]
pub type EADTBL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EADTBL` writer - GTADTRB Register A/D Conversion Start Request Extended Skipping Function Select"]
pub type EADTBL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTEITLI2_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - GTADTRA Register A/D Conversion Start Request Extended Skipping Function Select"]
    #[inline(always)]
    pub fn eadtal(&self) -> EADTAL_R {
        EADTAL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - GTADTRB Register A/D Conversion Start Request Extended Skipping Function Select"]
    #[inline(always)]
    pub fn eadtbl(&self) -> EADTBL_R {
        EADTBL_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - GTADTRA Register A/D Conversion Start Request Extended Skipping Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn eadtal(&mut self) -> EADTAL_W<0> {
        EADTAL_W::new(self)
    }
    #[doc = "Bits 4:6 - GTADTRB Register A/D Conversion Start Request Extended Skipping Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn eadtbl(&mut self) -> EADTBL_W<4> {
        EADTBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Extended Interrupt Skipping Setting Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gteitli2](index.html) module"]
pub struct GTEITLI2_SPEC;
impl crate::RegisterSpec for GTEITLI2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gteitli2::R](R) reader structure"]
impl crate::Readable for GTEITLI2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gteitli2::W](W) writer structure"]
impl crate::Writable for GTEITLI2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTEITLI2 to value 0"]
impl crate::Resettable for GTEITLI2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
