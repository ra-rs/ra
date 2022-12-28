#[doc = "Register `MRWCR1` reader"]
pub struct R(crate::R<MRWCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MRWCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MRWCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MRWCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MRWCR1` writer"]
pub struct W(crate::W<MRWCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MRWCR1_SPEC>;
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
impl From<crate::W<MRWCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MRWCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D1MRCMD0` reader - Memory map read command 0 setting"]
pub type D1MRCMD0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `D1MRCMD0` writer - Memory map read command 0 setting"]
pub type D1MRCMD0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MRWCR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `D1MRCMD1` reader - Memory map read command 1 setting"]
pub type D1MRCMD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `D1MRCMD1` writer - Memory map read command 1 setting"]
pub type D1MRCMD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MRWCR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `D1MWCMD0` reader - Memory map write command 0 setting"]
pub type D1MWCMD0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `D1MWCMD0` writer - Memory map write command 0 setting"]
pub type D1MWCMD0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MRWCR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `D1MWCMD1` reader - Memory map write command 1 setting"]
pub type D1MWCMD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `D1MWCMD1` writer - Memory map write command 1 setting"]
pub type D1MWCMD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MRWCR1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Memory map read command 0 setting"]
    #[inline(always)]
    pub fn d1mrcmd0(&self) -> D1MRCMD0_R {
        D1MRCMD0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Memory map read command 1 setting"]
    #[inline(always)]
    pub fn d1mrcmd1(&self) -> D1MRCMD1_R {
        D1MRCMD1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Memory map write command 0 setting"]
    #[inline(always)]
    pub fn d1mwcmd0(&self) -> D1MWCMD0_R {
        D1MWCMD0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Memory map write command 1 setting"]
    #[inline(always)]
    pub fn d1mwcmd1(&self) -> D1MWCMD1_R {
        D1MWCMD1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Memory map read command 0 setting"]
    #[inline(always)]
    #[must_use]
    pub fn d1mrcmd0(&mut self) -> D1MRCMD0_W<0> {
        D1MRCMD0_W::new(self)
    }
    #[doc = "Bits 8:15 - Memory map read command 1 setting"]
    #[inline(always)]
    #[must_use]
    pub fn d1mrcmd1(&mut self) -> D1MRCMD1_W<8> {
        D1MRCMD1_W::new(self)
    }
    #[doc = "Bits 16:23 - Memory map write command 0 setting"]
    #[inline(always)]
    #[must_use]
    pub fn d1mwcmd0(&mut self) -> D1MWCMD0_W<16> {
        D1MWCMD0_W::new(self)
    }
    #[doc = "Bits 24:31 - Memory map write command 1 setting"]
    #[inline(always)]
    #[must_use]
    pub fn d1mwcmd1(&mut self) -> D1MWCMD1_W<24> {
        D1MWCMD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Map Read/Write Command Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrwcr1](index.html) module"]
pub struct MRWCR1_SPEC;
impl crate::RegisterSpec for MRWCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mrwcr1::R](R) reader structure"]
impl crate::Readable for MRWCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mrwcr1::W](W) writer structure"]
impl crate::Writable for MRWCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MRWCR1 to value 0"]
impl crate::Resettable for MRWCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
