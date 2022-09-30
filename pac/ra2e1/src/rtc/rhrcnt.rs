#[doc = "Register `RHRCNT` reader"]
pub struct R(crate::R<RHRCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RHRCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RHRCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RHRCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RHRCNT` writer"]
pub struct W(crate::W<RHRCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RHRCNT_SPEC>;
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
impl From<crate::W<RHRCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RHRCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HR1` reader - 1-Hour Count"]
pub type HR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HR1` writer - 1-Hour Count"]
pub type HR1_W<'a, const O: u8> = crate::FieldWriter<'a, u8, RHRCNT_SPEC, u8, u8, 4, O>;
#[doc = "Field `HR10` reader - 10-Hour Count"]
pub type HR10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HR10` writer - 10-Hour Count"]
pub type HR10_W<'a, const O: u8> = crate::FieldWriter<'a, u8, RHRCNT_SPEC, u8, u8, 2, O>;
#[doc = "Field `PM` reader - AM/PM select for time counter setting."]
pub type PM_R = crate::BitReader<PM_A>;
#[doc = "AM/PM select for time counter setting.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PM_A {
    #[doc = "0: AM"]
    _0 = 0,
    #[doc = "1: PM"]
    _1 = 1,
}
impl From<PM_A> for bool {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as u8 != 0
    }
}
impl PM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PM_A {
        match self.bits {
            false => PM_A::_0,
            true => PM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PM_A::_1
    }
}
#[doc = "Field `PM` writer - AM/PM select for time counter setting."]
pub type PM_W<'a, const O: u8> = crate::BitWriter<'a, u8, RHRCNT_SPEC, PM_A, O>;
impl<'a, const O: u8> PM_W<'a, O> {
    #[doc = "AM"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PM_A::_0)
    }
    #[doc = "PM"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PM_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - 1-Hour Count"]
    #[inline(always)]
    pub fn hr1(&self) -> HR1_R {
        HR1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - 10-Hour Count"]
    #[inline(always)]
    pub fn hr10(&self) -> HR10_R {
        HR10_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - AM/PM select for time counter setting."]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-Hour Count"]
    #[inline(always)]
    pub fn hr1(&mut self) -> HR1_W<0> {
        HR1_W::new(self)
    }
    #[doc = "Bits 4:5 - 10-Hour Count"]
    #[inline(always)]
    pub fn hr10(&mut self) -> HR10_W<4> {
        HR10_W::new(self)
    }
    #[doc = "Bit 6 - AM/PM select for time counter setting."]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<6> {
        PM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hour Counter (in Calendar Count Mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rhrcnt](index.html) module"]
pub struct RHRCNT_SPEC;
impl crate::RegisterSpec for RHRCNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rhrcnt::R](R) reader structure"]
impl crate::Readable for RHRCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rhrcnt::W](W) writer structure"]
impl crate::Writable for RHRCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RHRCNT to value 0"]
impl crate::Resettable for RHRCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
