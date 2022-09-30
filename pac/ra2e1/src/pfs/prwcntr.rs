#[doc = "Register `PRWCNTR` reader"]
pub struct R(crate::R<PRWCNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRWCNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRWCNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRWCNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRWCNTR` writer"]
pub struct W(crate::W<PRWCNTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRWCNTR_SPEC>;
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
impl From<crate::W<PRWCNTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRWCNTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAIT` reader - Wait Cycle Control"]
pub type WAIT_R = crate::FieldReader<u8, WAIT_A>;
#[doc = "Wait Cycle Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAIT_A {
    #[doc = "0: Setting prohibited"]
    _00 = 0,
    #[doc = "1: Insert a 1-cycle wait"]
    _01 = 1,
    #[doc = "2: Insert a 2-cycle wait"]
    _10 = 2,
    #[doc = "3: Insert a 3-cycle wait"]
    _11 = 3,
}
impl From<WAIT_A> for u8 {
    #[inline(always)]
    fn from(variant: WAIT_A) -> Self {
        variant as _
    }
}
impl WAIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAIT_A {
        match self.bits {
            0 => WAIT_A::_00,
            1 => WAIT_A::_01,
            2 => WAIT_A::_10,
            3 => WAIT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WAIT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WAIT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WAIT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WAIT_A::_11
    }
}
#[doc = "Field `WAIT` writer - Wait Cycle Control"]
pub type WAIT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, PRWCNTR_SPEC, u8, WAIT_A, 2, O>;
impl<'a, const O: u8> WAIT_W<'a, O> {
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WAIT_A::_00)
    }
    #[doc = "Insert a 1-cycle wait"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WAIT_A::_01)
    }
    #[doc = "Insert a 2-cycle wait"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WAIT_A::_10)
    }
    #[doc = "Insert a 3-cycle wait"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WAIT_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Wait Cycle Control"]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Wait Cycle Control"]
    #[inline(always)]
    pub fn wait(&mut self) -> WAIT_W<0> {
        WAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Read Wait Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prwcntr](index.html) module"]
pub struct PRWCNTR_SPEC;
impl crate::RegisterSpec for PRWCNTR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [prwcntr::R](R) reader structure"]
impl crate::Readable for PRWCNTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prwcntr::W](W) writer structure"]
impl crate::Writable for PRWCNTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRWCNTR to value 0x01"]
impl crate::Resettable for PRWCNTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
