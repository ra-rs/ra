#[doc = "Register `DSR1` reader"]
pub struct R(crate::R<DSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSR1` writer"]
pub struct W(crate::W<DSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSR1_SPEC>;
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
impl From<crate::W<DSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DV1SZ` reader - Device 1 size setting"]
pub type DV1SZ_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DV1SZ` writer - Device 1 size setting"]
pub type DV1SZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DSR1_SPEC, u32, u32, 30, O>;
#[doc = "Field `DV1TYP` reader - Device 1 type setting"]
pub type DV1TYP_R = crate::FieldReader<u8, DV1TYP_A>;
#[doc = "Device 1 type setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DV1TYP_A {
    #[doc = "0: flash on device 1"]
    _00 = 0,
    #[doc = "1: RAM on device 1"]
    _01 = 1,
    #[doc = "2: no connection on device 1"]
    _10 = 2,
    #[doc = "3: forbidden"]
    _11 = 3,
}
impl From<DV1TYP_A> for u8 {
    #[inline(always)]
    fn from(variant: DV1TYP_A) -> Self {
        variant as _
    }
}
impl DV1TYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DV1TYP_A {
        match self.bits {
            0 => DV1TYP_A::_00,
            1 => DV1TYP_A::_01,
            2 => DV1TYP_A::_10,
            3 => DV1TYP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DV1TYP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DV1TYP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DV1TYP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DV1TYP_A::_11
    }
}
#[doc = "Field `DV1TYP` writer - Device 1 type setting"]
pub type DV1TYP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DSR1_SPEC, u8, DV1TYP_A, 2, O>;
impl<'a, const O: u8> DV1TYP_W<'a, O> {
    #[doc = "flash on device 1"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DV1TYP_A::_00)
    }
    #[doc = "RAM on device 1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DV1TYP_A::_01)
    }
    #[doc = "no connection on device 1"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DV1TYP_A::_10)
    }
    #[doc = "forbidden"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DV1TYP_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:29 - Device 1 size setting"]
    #[inline(always)]
    pub fn dv1sz(&self) -> DV1SZ_R {
        DV1SZ_R::new(self.bits & 0x3fff_ffff)
    }
    #[doc = "Bits 30:31 - Device 1 type setting"]
    #[inline(always)]
    pub fn dv1typ(&self) -> DV1TYP_R {
        DV1TYP_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:29 - Device 1 size setting"]
    #[inline(always)]
    #[must_use]
    pub fn dv1sz(&mut self) -> DV1SZ_W<0> {
        DV1SZ_W::new(self)
    }
    #[doc = "Bits 30:31 - Device 1 type setting"]
    #[inline(always)]
    #[must_use]
    pub fn dv1typ(&mut self) -> DV1TYP_W<30> {
        DV1TYP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Size Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsr1](index.html) module"]
pub struct DSR1_SPEC;
impl crate::RegisterSpec for DSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsr1::R](R) reader structure"]
impl crate::Readable for DSR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsr1::W](W) writer structure"]
impl crate::Writable for DSR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSR1 to value 0"]
impl crate::Resettable for DSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
