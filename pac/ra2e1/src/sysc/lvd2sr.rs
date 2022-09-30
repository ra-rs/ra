#[doc = "Register `LVD2SR` reader"]
pub struct R(crate::R<LVD2SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LVD2SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LVD2SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LVD2SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LVD2SR` writer"]
pub struct W(crate::W<LVD2SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LVD2SR_SPEC>;
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
impl From<crate::W<LVD2SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LVD2SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DET` reader - Voltage Monitor 2 Voltage Variation Detection Flag"]
pub type DET_R = crate::BitReader<DET_A>;
#[doc = "Voltage Monitor 2 Voltage Variation Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DET_A {
    #[doc = "0: Not detected"]
    _0 = 0,
    #[doc = "1: Vdet2 crossing is detected"]
    _1 = 1,
}
impl From<DET_A> for bool {
    #[inline(always)]
    fn from(variant: DET_A) -> Self {
        variant as u8 != 0
    }
}
impl DET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DET_A {
        match self.bits {
            false => DET_A::_0,
            true => DET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DET_A::_1
    }
}
#[doc = "Field `DET` writer - Voltage Monitor 2 Voltage Variation Detection Flag"]
pub type DET_W<'a, const O: u8> = crate::BitWriter<'a, u8, LVD2SR_SPEC, DET_A, O>;
impl<'a, const O: u8> DET_W<'a, O> {
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DET_A::_0)
    }
    #[doc = "Vdet2 crossing is detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DET_A::_1)
    }
}
#[doc = "Field `MON` reader - Voltage Monitor 2 Signal Monitor Flag"]
pub type MON_R = crate::BitReader<MON_A>;
#[doc = "Voltage Monitor 2 Signal Monitor Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MON_A {
    #[doc = "0: VCC < Vdet2"]
    _0 = 0,
    #[doc = "1: VCC>= Vdet2 or MON is disabled"]
    _1 = 1,
}
impl From<MON_A> for bool {
    #[inline(always)]
    fn from(variant: MON_A) -> Self {
        variant as u8 != 0
    }
}
impl MON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MON_A {
        match self.bits {
            false => MON_A::_0,
            true => MON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MON_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Voltage Monitor 2 Voltage Variation Detection Flag"]
    #[inline(always)]
    pub fn det(&self) -> DET_R {
        DET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Voltage Monitor 2 Signal Monitor Flag"]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage Monitor 2 Voltage Variation Detection Flag"]
    #[inline(always)]
    pub fn det(&mut self) -> DET_W<0> {
        DET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voltage Monitor 2 Circuit Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvd2sr](index.html) module"]
pub struct LVD2SR_SPEC;
impl crate::RegisterSpec for LVD2SR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lvd2sr::R](R) reader structure"]
impl crate::Readable for LVD2SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lvd2sr::W](W) writer structure"]
impl crate::Writable for LVD2SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LVD2SR to value 0x02"]
impl crate::Resettable for LVD2SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
