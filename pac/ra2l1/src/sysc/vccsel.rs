#[doc = "Register `VCCSEL` reader"]
pub struct R(crate::R<VCCSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VCCSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VCCSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VCCSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VCCSEL` writer"]
pub struct W(crate::W<VCCSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VCCSEL_SPEC>;
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
impl From<crate::W<VCCSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VCCSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VCCSEL` reader - DCDC Working Voltage Level Selection"]
pub type VCCSEL_R = crate::FieldReader<u8, VCCSEL_A>;
#[doc = "DCDC Working Voltage Level Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VCCSEL_A {
    #[doc = "0: 2.7 V =< VCC < 3.6 V"]
    _00 = 0,
    #[doc = "1: 3.6 V =< VCC < 4.5 V"]
    _01 = 1,
    #[doc = "2: 4.5 V =< VCC â\u{89}¤ 5.5 V"]
    _10 = 2,
    #[doc = "3: 2.4 V =< VCC < 2.7 V"]
    _11 = 3,
}
impl From<VCCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VCCSEL_A) -> Self {
        variant as _
    }
}
impl VCCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCCSEL_A {
        match self.bits {
            0 => VCCSEL_A::_00,
            1 => VCCSEL_A::_01,
            2 => VCCSEL_A::_10,
            3 => VCCSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == VCCSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == VCCSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == VCCSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == VCCSEL_A::_11
    }
}
#[doc = "Field `VCCSEL` writer - DCDC Working Voltage Level Selection"]
pub type VCCSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, VCCSEL_SPEC, u8, VCCSEL_A, 2, O>;
impl<'a, const O: u8> VCCSEL_W<'a, O> {
    #[doc = "2.7 V =< VCC < 3.6 V"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(VCCSEL_A::_00)
    }
    #[doc = "3.6 V =< VCC < 4.5 V"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(VCCSEL_A::_01)
    }
    #[doc = "4.5 V =< VCC â\u{89}¤ 5.5 V"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(VCCSEL_A::_10)
    }
    #[doc = "2.4 V =< VCC < 2.7 V"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(VCCSEL_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - DCDC Working Voltage Level Selection"]
    #[inline(always)]
    pub fn vccsel(&self) -> VCCSEL_R {
        VCCSEL_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - DCDC Working Voltage Level Selection"]
    #[inline(always)]
    #[must_use]
    pub fn vccsel(&mut self) -> VCCSEL_W<0> {
        VCCSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voltage Level Selection Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vccsel](index.html) module"]
pub struct VCCSEL_SPEC;
impl crate::RegisterSpec for VCCSEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vccsel::R](R) reader structure"]
impl crate::Readable for VCCSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vccsel::W](W) writer structure"]
impl crate::Writable for VCCSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VCCSEL to value 0"]
impl crate::Resettable for VCCSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
