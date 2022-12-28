#[doc = "Register `CS%sWCR1` reader"]
pub struct R(crate::R<CSWCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSWCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSWCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSWCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CS%sWCR1` writer"]
pub struct W(crate::W<CSWCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSWCR1_SPEC>;
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
impl From<crate::W<CSWCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSWCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSPWWAIT` reader - Page Write Cycle Wait Select"]
pub type CSPWWAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSPWWAIT` writer - Page Write Cycle Wait Select"]
pub type CSPWWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSWCR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `CSPRWAIT` reader - Page Read Cycle Wait Select"]
pub type CSPRWAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSPRWAIT` writer - Page Read Cycle Wait Select"]
pub type CSPRWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSWCR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `CSWWAIT` reader - Normal Write Cycle Wait Select"]
pub type CSWWAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSWWAIT` writer - Normal Write Cycle Wait Select"]
pub type CSWWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSWCR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `CSRWAIT` reader - Normal Read Cycle Wait Select"]
pub type CSRWAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSRWAIT` writer - Normal Read Cycle Wait Select"]
pub type CSRWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSWCR1_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:2 - Page Write Cycle Wait Select"]
    #[inline(always)]
    pub fn cspwwait(&self) -> CSPWWAIT_R {
        CSPWWAIT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Page Read Cycle Wait Select"]
    #[inline(always)]
    pub fn csprwait(&self) -> CSPRWAIT_R {
        CSPRWAIT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:20 - Normal Write Cycle Wait Select"]
    #[inline(always)]
    pub fn cswwait(&self) -> CSWWAIT_R {
        CSWWAIT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Normal Read Cycle Wait Select"]
    #[inline(always)]
    pub fn csrwait(&self) -> CSRWAIT_R {
        CSRWAIT_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Page Write Cycle Wait Select"]
    #[inline(always)]
    #[must_use]
    pub fn cspwwait(&mut self) -> CSPWWAIT_W<0> {
        CSPWWAIT_W::new(self)
    }
    #[doc = "Bits 8:10 - Page Read Cycle Wait Select"]
    #[inline(always)]
    #[must_use]
    pub fn csprwait(&mut self) -> CSPRWAIT_W<8> {
        CSPRWAIT_W::new(self)
    }
    #[doc = "Bits 16:20 - Normal Write Cycle Wait Select"]
    #[inline(always)]
    #[must_use]
    pub fn cswwait(&mut self) -> CSWWAIT_W<16> {
        CSWWAIT_W::new(self)
    }
    #[doc = "Bits 24:28 - Normal Read Cycle Wait Select"]
    #[inline(always)]
    #[must_use]
    pub fn csrwait(&mut self) -> CSRWAIT_W<24> {
        CSRWAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CS%s Wait Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cswcr1](index.html) module"]
pub struct CSWCR1_SPEC;
impl crate::RegisterSpec for CSWCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cswcr1::R](R) reader structure"]
impl crate::Readable for CSWCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cswcr1::W](W) writer structure"]
impl crate::Writable for CSWCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CS%sWCR1 to value 0x0707_0707"]
impl crate::Resettable for CSWCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0707_0707;
}
