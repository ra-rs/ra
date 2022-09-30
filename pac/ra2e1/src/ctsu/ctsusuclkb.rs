#[doc = "Register `CTSUSUCLKB` reader"]
pub struct R(crate::R<CTSUSUCLKB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUSUCLKB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUSUCLKB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUSUCLKB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUSUCLKB` writer"]
pub struct W(crate::W<CTSUSUCLKB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUSUCLKB_SPEC>;
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
impl From<crate::W<CTSUSUCLKB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUSUCLKB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUADJ2` reader - CTSU SUCLK Frequency Adjustment"]
pub type SUADJ2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUADJ2` writer - CTSU SUCLK Frequency Adjustment"]
pub type SUADJ2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTSUSUCLKB_SPEC, u8, u8, 8, O>;
#[doc = "Field `SUMULTI2` reader - CTSU SUCLK Multiplier Rate Setting"]
pub type SUMULTI2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUMULTI2` writer - CTSU SUCLK Multiplier Rate Setting"]
pub type SUMULTI2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTSUSUCLKB_SPEC, u8, u8, 8, O>;
#[doc = "Field `SUADJ3` reader - CTSU SUCLK Frequency Adjustment"]
pub type SUADJ3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUADJ3` writer - CTSU SUCLK Frequency Adjustment"]
pub type SUADJ3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTSUSUCLKB_SPEC, u8, u8, 8, O>;
#[doc = "Field `SUMULTI3` reader - CTSU SUCLK Multiplier Rate Setting"]
pub type SUMULTI3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUMULTI3` writer - CTSU SUCLK Multiplier Rate Setting"]
pub type SUMULTI3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTSUSUCLKB_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - CTSU SUCLK Frequency Adjustment"]
    #[inline(always)]
    pub fn suadj2(&self) -> SUADJ2_R {
        SUADJ2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CTSU SUCLK Multiplier Rate Setting"]
    #[inline(always)]
    pub fn sumulti2(&self) -> SUMULTI2_R {
        SUMULTI2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CTSU SUCLK Frequency Adjustment"]
    #[inline(always)]
    pub fn suadj3(&self) -> SUADJ3_R {
        SUADJ3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CTSU SUCLK Multiplier Rate Setting"]
    #[inline(always)]
    pub fn sumulti3(&self) -> SUMULTI3_R {
        SUMULTI3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CTSU SUCLK Frequency Adjustment"]
    #[inline(always)]
    pub fn suadj2(&mut self) -> SUADJ2_W<0> {
        SUADJ2_W::new(self)
    }
    #[doc = "Bits 8:15 - CTSU SUCLK Multiplier Rate Setting"]
    #[inline(always)]
    pub fn sumulti2(&mut self) -> SUMULTI2_W<8> {
        SUMULTI2_W::new(self)
    }
    #[doc = "Bits 16:23 - CTSU SUCLK Frequency Adjustment"]
    #[inline(always)]
    pub fn suadj3(&mut self) -> SUADJ3_W<16> {
        SUADJ3_W::new(self)
    }
    #[doc = "Bits 24:31 - CTSU SUCLK Multiplier Rate Setting"]
    #[inline(always)]
    pub fn sumulti3(&mut self) -> SUMULTI3_W<24> {
        SUMULTI3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Sensor Unit Clock Control Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsusuclkb](index.html) module"]
pub struct CTSUSUCLKB_SPEC;
impl crate::RegisterSpec for CTSUSUCLKB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctsusuclkb::R](R) reader structure"]
impl crate::Readable for CTSUSUCLKB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsusuclkb::W](W) writer structure"]
impl crate::Writable for CTSUSUCLKB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTSUSUCLKB to value 0"]
impl crate::Resettable for CTSUSUCLKB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
